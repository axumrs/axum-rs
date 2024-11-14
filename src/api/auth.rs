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
) -> Result<resp::JsonResp<resp::AuthResp<model::user::User>>> {
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
        None => return Err(Error::new("用户名/密码错误").into()),
    };

    // 验证状态
    match &user.status {
        &model::user::Status::Pending => return Err(Error::new("Pending").into()),
        &model::user::Status::Freezed => return Err(Error::new("用户被冻结").into()),
        &model::user::Status::Actived => {
            // pass
        }
    }

    // 验证密码
    if !utils::password::verify(&frm.password, &user.password).map_err(log_error(handler_name))? {
        return Err(Error::new("用户名/密码错误").into());
    }

    // 订阅
    let user = match &user.kind {
        &model::user::Kind::Subscriber | &model::user::Kind::YearlySubscriber => {
            let u = if Local::now() >= user.sub_exp {
                let user = model::user::User {
                    kind: model::user::Kind::Normal,
                    allow_device_num: 1,
                    session_exp: state.cfg.session.default_timeout as i16,
                    ..user
                };
                if let Err(e) = user.update(&mut *tx).await {
                    tx.rollback()
                        .await
                        .map_err(Error::from)
                        .map_err(log_error(handler_name))?;
                    return Err(e.into());
                }
                user
            } else {
                user
            };
            u
        }
        _ => user,
    };

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
        user_id: user.id.clone(),
        token,
        is_admin: false,
        dateline,
        ip: ip.clone(),
        ua: user_agent.clone(),
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

    // 登录日志
    let lglog = model::login_log::LoginLog {
        id: utils::id::new(),
        user_id: user.id.clone(),
        dateline,
        ip,
        user_agent,
    };
    if let Err(e) = lglog.insert(&mut *tx).await {
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

    Ok(resp::ok(resp::AuthResp {
        user,
        token: session.token,
        expire_time: session.expire_time,
    }))
}

pub async fn admin_login(
    State(state): State<ArcAppState>,
    mid::IpAndUserAgent {
        ip_location,
        ip,
        user_agent,
    }: mid::IpAndUserAgent,
    Json(frm): Json<form::auth::AdminLoginForm>,
) -> Result<resp::JsonResp<resp::AuthResp<model::admin::Admin>>> {
    let handler_name = "auth/admin-login";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 人机验证
    if !captcha::verify_turnstile(&state.cfg, &frm.captcha)
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

    let admin = match model::admin::Admin::find(
        &mut *tx,
        &model::admin::AdminFindFilter {
            by: model::admin::AdminFindBy::Username(frm.username.clone()),
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

    let admin = match admin {
        Some(v) => v,
        None => return Err(Error::new("用户名/密码错误").into()),
    };

    if !utils::password::verify(&frm.password, &admin.password).map_err(log_error(handler_name))? {
        return Err(Error::new("用户名/密码错误").into());
    }

    // 登录
    let id = utils::id::new();
    let (token, dateline) = utils::session::token(&admin.id, &state.cfg.session.secret_key, true)
        .map_err(log_error(handler_name))?;
    let expire_time = dateline + chrono::Duration::minutes(state.cfg.session.admin_timeout as i64);
    let loc = utils::str::fixlen(&ip_location, 100).to_string();
    let session = model::session::Session {
        id,
        user_id: admin.id.clone(),
        token,
        is_admin: true,
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
    Ok(resp::ok(resp::AuthResp {
        user: admin,
        token: session.token,
        expire_time: session.expire_time,
    }))
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

    // 人机验证
    if !captcha::verify_hcaptcha(&state.cfg, &frm.captcha)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::new("人机验证失败")).map_err(log_error(handler_name));
    }

    let p = get_pool(&state);

    let user = model::user::UserBuilder::new(frm.user.email, frm.user.nickname, frm.user.password)
        .status(model::user::Status::Pending)
        .kind(model::user::Kind::Normal)
        .dateline_now()
        .allow_device_num(1)
        .session_exp(state.cfg.session.default_timeout as i16)
        .build()?;

    let model::user::User { id, .. } = service::user::add(&*p, user)
        .await
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::IDResp { id }))
}

pub async fn send_code(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::auth::SendCodeForm>,
) -> Result<resp::JsonResp<()>> {
    let handler_name = "auth/send-code";
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
            kind: frm.kind,
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
        subject: "AXUM中文网".to_string(),
        body: format!("你在AXUM中文网的验证码是: {}，请在5分钟内完成验证", &code),
        to: frm.email.clone(),
    };
    tokio::spawn(mail::send(mc, d));

    Ok(resp::ok(()))
}

pub async fn active(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::auth::ActiveForm>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "auth/active";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    let p = get_pool(&state);

    // 人机验证
    if !captcha::verify_turnstile(&state.cfg, &frm.captcha)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::new("人机验证失败")).map_err(log_error(handler_name));
    }

    // 查找验证码
    let ac = match model::activation_code::ActivationCode::find(
        &*p,
        &model::activation_code::ActivationCodeFindFilter {
            id: None,
            email: Some(frm.email.clone()),
            code: Some(frm.activation_code),
            kind: Some(frm.kind),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("验证码错误")),
    };

    // 激活
    let user = match model::user::User::find(
        &*p,
        &model::user::UserFindFilter {
            by: model::user::UserFindBy::Email(frm.email),
            status: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("用户不存在")),
    };

    match &user.status {
        &model::user::Status::Actived => return Err(Error::new("用户已激活，无需再次激活")),
        &model::user::Status::Freezed => return Err(Error::new("用户已被冻结")),
        _ => {}
    }

    let aff = model::user::User::update_status(&*p, &model::user::Status::Actived, &user.id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 删除验证码
    model::activation_code::ActivationCode::real_del(&*p, &ac.id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn reset_password(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::auth::ResetPasswordForm>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "auth/reset-password";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if frm.password != frm.re_password {
        return Err(Error::new("两次输入的密码不一致")).map_err(log_error(handler_name));
    }

    // 人机验证
    if !captcha::verify_turnstile(&state.cfg, &frm.captcha)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::new("人机验证失败")).map_err(log_error(handler_name));
    }

    let p = get_pool(&state);

    // 查找验证码
    let ac = match model::activation_code::ActivationCode::find(
        &*p,
        &model::activation_code::ActivationCodeFindFilter {
            id: None,
            email: Some(frm.email.clone()),
            code: Some(frm.activation_code),
            kind: Some(model::activation_code::Kind::ResetPassword),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("验证码错误")),
    };

    // 修改密码
    let user = match model::user::User::find(
        &*p,
        &model::user::UserFindFilter {
            by: model::user::UserFindBy::Email(frm.email),
            status: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("用户不存在")),
    };

    let password = utils::password::hash(&frm.password).map_err(log_error(handler_name))?;

    let aff = model::user::User::update_password(&*p, &password, &user.id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 删除验证码
    model::activation_code::ActivationCode::real_del(&*p, &ac.id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}
