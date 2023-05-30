use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};

use crate::{
    db::{user_purchased_service, Paginate},
    form::user_purchased_service as form,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    Error, JsonRespone, Response, Result,
};

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u64>,
) -> Result<JsonRespone<model::UserPurchasedServiceFull>> {
    let handler_name = "admin/user_purchased_service/find";
    let conn = get_conn(&state);

    let ups = user_purchased_service::find(&conn, id, None, None)
        .await
        .map_err(log_error(handler_name))?;
    match ups {
        Some(ups) => Ok(Response::ok(ups).to_json()),
        None => Err(Error::not_found("不存在的记录")),
    }
}

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<form::List>,
) -> Result<JsonRespone<Paginate<model::UserPurchasedServiceFull>>> {
    let handler_name = "admin/user_purchased_service/list";
    let conn = get_conn(&state);

    let p = user_purchased_service::list(
        &conn,
        &model::UserPurchasedServiceFullListWith {
            user_id: frm.user_id,
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
