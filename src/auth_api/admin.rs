use std::sync::Arc;

use axum::{Extension, Json};

use crate::{
    captcha::Captcha,
    db::admin,
    form::auth as form,
    handler_helper::{get_conn, log_error},
    jwt,
    model::{self, State},
    password, Error, JsonRespone, Response, Result,
};

pub async fn login(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::AdminLogin>,
) -> Result<JsonRespone<jwt::AuthBody>> {
    let handler_name = "admin/auth/login";

    let cpt = Captcha::new_hcaptcha(&state.cfg.hcaptcha.secret_key);
    if !cpt
        .verify(&frm.response)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::captcha_failed());
    }

    let conn = get_conn(&state);

    let adm = admin::find(&conn, &model::AdminFindBy::Username(&frm.username))
        .await
        .map_err(log_error(handler_name))?;

    if adm.is_none() {
        return Err(Error::not_found("用户名或密码错误1"));
    }

    let adm = adm.unwrap();
    if !password::verify(&frm.password, &adm.password)? {
        return Err(Error::not_found("用户名或密码错误2"));
    }

    let auth_body = jwt::token::encode(
        &state.cfg.admin_jwt,
        jwt::AdminClaimsData {
            id: adm.id,
            username: adm.username,
        },
    )?;

    Ok(Response::ok(auth_body).to_json())
}
