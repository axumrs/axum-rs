use axum::{extract::State, Json};
use chrono::Local;
use validator::Validate;

use crate::{captcha, form, mail, model, resp, service, utils, ArcAppState, Error, Result};

use super::{get_pool, log_error};

pub async fn login(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::auth::LoginForm>,
) -> Result<resp::JsonResp<()>> {
    let handler_name = "auth/login";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 人机验证

    let _p = get_pool(&state);
    Ok(resp::ok(()))
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
    service::user::add(&*p, user)
        .await
        .map_err(log_error(handler_name))?;

    model::activation_code::ActivationCode::real_del(&*p, &ac.id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::IDResp { id: "()".into() }))
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
