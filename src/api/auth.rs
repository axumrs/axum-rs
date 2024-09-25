use axum::{extract::State, Json};
use validator::Validate;

use crate::{form, resp, service, ArcAppState, Error, Result};

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
    let handler_name = "auth/login";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if frm.user.password != frm.user.re_password {
        return Err(Error::new("两次输入的密码不一致")).map_err(log_error(handler_name));
    }

    // 人机验证

    let _p = get_pool(&state);

    // 发送邮件

    Ok(resp::ok(resp::IDResp { id: "()".into() }))
}
