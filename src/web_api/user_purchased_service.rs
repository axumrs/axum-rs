use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};

use crate::{
    db::{user_purchased_service, Paginate},
    handler_helper::{get_conn, log_error},
    middleware::UserAuth,
    model::{self, State},
    Error, JsonRespone, Response, Result,
};

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u64>,
    UserAuth(claims): UserAuth,
) -> Result<JsonRespone<model::UserPurchasedServiceFull>> {
    let handler_name = "web/user_purchased_service/find";
    let conn = get_conn(&state);

    let ups = user_purchased_service::find(&conn, id, None, Some(claims.id))
        .await
        .map_err(log_error(handler_name))?;
    match ups {
        Some(ups) => Ok(Response::ok(ups).to_json()),
        None => Err(Error::not_found("不存在的记录")),
    }
}

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<crate::form::PaginateForm>,
    UserAuth(claims): UserAuth,
) -> Result<JsonRespone<Paginate<model::UserPurchasedServiceFull>>> {
    let handler_name = "web/user_purchased_service/list";
    let conn = get_conn(&state);

    let p = user_purchased_service::list(
        &conn,
        &model::UserPurchasedServiceFullListWith {
            user_id: Some(claims.id),
            pw: model::PaginateWith {
                page: frm.page,
                page_size: frm.page_size,
            },
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}
