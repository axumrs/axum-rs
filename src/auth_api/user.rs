use std::sync::Arc;

use axum::{Extension, Json, TypedHeader};
use headers::UserAgent;
use validator::Validate;

use crate::{
    captcha::Captcha,
    db::user,
    form::auth as form,
    handler_helper::{get_conn, log_error},
    jwt::{self, AuthBody},
    model::{self, State},
    rdb, uap, uuid, Error, IDResponse, JsonRespone, Response, Result,
};

pub async fn register(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::UserRegister>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "auth/user/register";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if !Captcha::new_hcaptcha(&state.cfg.hcaptcha.secret_key)
        .verify(&frm.response)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::captcha_failed());
    }

    let conn = get_conn(&state);

    let id = user::add(
        &conn,
        &model::User {
            email: frm.email,
            nickname: frm.nickname,
            password: frm.password,
            status: (&state.cfg.users).register_default_status,
            dateline: chrono::Local::now(),
            allow_device_num: 1,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn login(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Json(frm): Json<form::UserLogin>,
) -> Result<JsonRespone<AuthBody>> {
    let handler_name = "auth/user/login";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if !Captcha::new_hcaptcha(&state.cfg.hcaptcha.secret_key)
        .verify(&frm.response)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::captcha_failed());
    }

    // rds: 允许的设备数
    let user_allow_drive = rdb::user::get_allow_drive(&state.rds, &state.cfg, &frm.email)
        .await
        .map_err(log_error(handler_name))?;
    tracing::debug!("allow drive: {}", user_allow_drive);

    // rds: JWT过期时间
    let user_jwt_exp = rdb::user::get_jwt_exp(&state.rds, &state.cfg, &frm.email)
        .await
        .map_err(log_error(handler_name))?;

    // rds: 在线设备数
    let user_online_count = rdb::user::count_online(&state.rds, &state.cfg, &frm.email)
        .await
        .map_err(log_error(handler_name))?;
    tracing::debug!("user online count: {}", user_online_count);

    if user_online_count >= user_allow_drive {
        return Err(Error::no_available_device());
    }

    let available_device_num: u8 = user_allow_drive - user_online_count;

    // db
    let conn = get_conn(&state);
    let uai = uap::parse(user_agent.as_str()).map_err(log_error(handler_name))?;
    let login_meta = model::UserLoginMeta {
        email: frm.email,
        password: frm.password,
        ip: frm.ip,
        uai: uai.clone(),
        ua: user_agent.to_string(),
    };
    let (u, login_id) = user::login(&conn, &login_meta)
        .await
        .map_err(log_error(handler_name))?;

    let email = u.email.clone();
    let allow_driver = u.allow_device_num;
    let online_id = uuid::new();
    let user_jwt_exp = if user_jwt_exp == 0 {
        //rds
        if u.jwt_exp == 0 {
            // db
            (&state.cfg.user_jwt).expired
        } else {
            u.jwt_exp as u32
        }
    } else {
        user_jwt_exp as u32
    };

    let cd = jwt::UserClaimsData {
        id: u.id,
        email: u.email,
        nickname: u.nickname,
        status: u.status,
        dateline: u.dateline,
        types: u.types,
        sub_exp: u.sub_exp,
        points: u.points,
        allow_device_num: u.allow_device_num,
        available_device_num,
        login_id,
        uai,
        online_id: online_id.clone(),
        ip: login_meta.ip.clone(),
    };

    // rds: 添加到在线设备数、写入登录设置
    rdb::user::set_allow_drive(&state.rds, &state.cfg, &email, allow_driver)
        .await
        .map_err(log_error(handler_name))?;
    rdb::user::set_jwt_exp(&state.rds, &state.cfg, &email, user_jwt_exp as u8)
        .await
        .map_err(log_error(handler_name))?;
    rdb::user::set_online(
        &state.rds,
        &state.cfg,
        &email,
        &cd,
        user_jwt_exp as u32,
        &online_id,
    )
    .await
    .map_err(log_error(handler_name))?;

    // token
    let auth_body = jwt::token::encode(
        &crate::config::Jwt {
            expired: user_jwt_exp as u32,
            ..(&state.cfg.user_jwt).clone()
        },
        cd,
    )
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(auth_body).to_json())
}
