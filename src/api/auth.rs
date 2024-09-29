use axum::{extract::State, Json};
use chrono::Local;
use validator::Validate;

use crate::{captcha, form, mail, mid, model, resp, service, utils, ArcAppState, Error, Result};

use super::{get_pool, log_error};

pub async fn login(
    State(state): State<ArcAppState>,
    mid::IpAndUserAgent {
        ip_location,
        ip,
        user_agent,
    }: mid::IpAndUserAgent,
    Json(frm): Json<form::auth::LoginForm>,
) -> Result<resp::JsonResp<String>> {
    let handler_name = "auth/login";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 人机验证
    if !captcha::verify_hcaptcha(&state.cfg, &frm.captcha)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::new("人机验证失败")).map_err(log_error(handler_name));
    }

    let p = get_pool(&state);
    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 查找用户
    let user = match model::user::User::find(
        &mut *tx,
        &model::user::UserFindFilter {
            by: model::user::UserFindBy::Email(frm.email.clone()),
            status: None,
        },
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into());
        }
    };

    let user = match user {
        Some(v) => v,
        None => return Err(Error::new("不存在的用户").into()),
    };

    // 验证状态
    match &user.status {
        &model::user::Status::Pending => return Err(Error::new("用户尚未激活").into()),
        &model::user::Status::Freezed => return Err(Error::new("用户被冻结").into()),
        &model::user::Status::Actived => {
            // pass
        }
    }

    // 验证密码
    if !utils::password::verify(&frm.password, &user.password).map_err(log_error(handler_name))? {
        return Err(Error::new("密码错误").into());
    }

    // 已登录数量
    let count: (i64,) = match sqlx::query_as(
        "SELECT count(*) FROM sessions WHERE user_id = $1 AND is_admin=false AND expire_time>=$2",
    )
    .bind(&user.id)
    .bind(&Local::now())
    .fetch_one(&mut *tx)
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into());
        }
    };

    if count.0 >= user.allow_device_num as i64 {
        return Err(Error::new("登录设备过多").into());
    }

    // 登录
    let id = utils::id::new();
    let (token, dateline) = utils::session::token(&user.id, &state.cfg.session.secret_key, false)
        .map_err(log_error(handler_name))?;
    let expire_time = dateline + chrono::Duration::minutes(user.session_exp as i64);
    let loc = utils::str::fixlen(&ip_location, 100).to_string();
    let session = model::session::Session {
        id,
        user_id: user.id,
        token,
        is_admin: false,
        dateline,
        ip,
        ua: user_agent,
        loc,
        expire_time,
    };

    if let Err(e) = session.insert(&mut *tx).await {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into());
    }

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(session.token))
}
pub async fn register(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::auth::RegisterForm>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "auth/register";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if frm.user.password != frm.user.re_password {
        return Err(Error::new("两次输入的密码不一致")).map_err(log_error(handler_name));
    }

    let p = get_pool(&state);

    // 验证码
    let ac = match service::activation_code::get(
        &*p,
        &frm.user.email,
        model::activation_code::Kind::Register,
        Some(frm.captcha.clone()),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            return Err(e.into());
        }
    };

    let ac = match ac {
        Some(v) => v,
        None => {
            return Err(Error::new("不存在的验证码")).map_err(log_error(handler_name))?;
        }
    };

    if ac.email != frm.user.email {
        return Err(Error::new("验证码错误")).map_err(log_error(handler_name));
    }

    let user = model::user::UserBuilder::new(frm.user.email, frm.user.nickname, frm.user.password)
        .status(model::user::Status::Actived)
        .kind(model::user::Kind::Normal)
        .dateline_now()
        .allow_device_num(1)
        .session_exp(state.cfg.session.default_timeout as i16)
        .build()?;

    let model::user::User { id, .. } = service::user::add(&*p, user)
        .await
        .map_err(log_error(handler_name))?;

    model::activation_code::ActivationCode::real_del(&*p, &ac.id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::IDResp { id }))
}

pub async fn register_send_code(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::auth::RegisterSendCodeForm>,
) -> Result<resp::JsonResp<()>> {
    let handler_name = "auth/register-send-code";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 人机验证
    if !captcha::verify_hcaptcha(&state.cfg, &frm.captcha)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::new("人机验证失败")).map_err(log_error(handler_name));
    }

    // 生成验证码
    let code = utils::str::activation_code();
    let p = get_pool(&state);

    // 保存验证码
    service::activation_code::add(
        &*p,
        model::activation_code::ActivationCode {
            email: frm.email.clone(),
            code: code.clone(),
            kind: model::activation_code::Kind::Register,
            dateline: Local::now(),
            ..Default::default()
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    // 发送邮件
    let mc = state.cfg.clone();
    let d = mail::Data {
        subject: "欢迎注册AXUM中文网".to_string(),
        body: format!(
            "你在AXUM中文网的注册验证码是: {}，请在5分钟内完成验证",
            &code
        ),
        to: frm.email.clone(),
    };
    tokio::spawn(mail::send(mc, d));

    Ok(resp::ok(()))
}
