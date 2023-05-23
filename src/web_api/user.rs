use std::sync::Arc;

use axum::Extension;

use crate::{
    handler_helper::log_error, jwt, middleware::UserAuth, model::State, rdb, JsonRespone, Response,
    Result,
};

pub async fn index(Extension(state): Extension<Arc<State>>) -> Result<JsonRespone<String>> {
    let handler_name = "web/user/index";

    Ok(Response::ok(handler_name.to_string()).to_json())
}

pub async fn subscribe(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<String>> {
    let handler_name = "web/user/subscribe";
    // tracing::debug!("{:?}", cd);
    Ok(Response::ok(format!("{:?}", cd)).to_json())
}
pub async fn logout(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<jwt::UserClaimsData>> {
    let handler_name = "web/user/logout";
    rdb::user::del_online(&state.rds, &state.cfg, &cd.email, &cd.online_id)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(cd).to_json())
}
