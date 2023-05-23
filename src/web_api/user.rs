use std::sync::Arc;

use axum::{extract::Query, Extension};

use crate::{
    db::{user_login_log, Paginate},
    handler_helper::{get_conn, log_error},
    jwt,
    middleware::UserAuth,
    model::{self, State},
    rdb, JsonRespone, Response, Result,
};

pub async fn online_derive(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<Vec<jwt::UserClaimsData>>> {
    let handler_name = "web/user/online_derive";

    let list = rdb::user::get_online_list(&state.rds, &state.cfg, &cd.email)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(list).to_json())
}

pub async fn login_log(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
    Query(frm): Query<crate::form::PaginateForm>,
) -> Result<JsonRespone<Paginate<model::UserLoginLogFull>>> {
    let handler_name = "web/user/login_log";

    let conn = get_conn(&state);
    let p = user_login_log::list(
        &conn,
        &model::PaginateWith {
            page: frm.page,
            page_size: frm.page_size,
        },
        cd.id,
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
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
