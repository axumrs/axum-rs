use std::sync::Arc;

use axum::{extract::Path, Extension};

use crate::{
    db::{order, Paginate},
    handler_helper::{get_conn, log_error},
    model::{self, State},
    Error, JsonRespone, Response, Result,
};

pub async fn list(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonRespone<Paginate<model::OrderFullWithUser>>> {
    let handler_name = "admin/order/list";
    let conn = get_conn(&state);
    let p = order::list_full_with_user(
        &conn,
        &model::OrderListWith {
            page: 0,
            page_size: 30,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}

pub async fn find(
    Extension(state): Extension<Arc<State>>,

    Path(id): Path<u64>,
) -> Result<JsonRespone<model::OrderFullWithUser>> {
    let handler_name = "admin/order/find";

    let conn = get_conn(&state);

    let o = order::find_with_user(&conn, id, None, None)
        .await
        .map_err(log_error(handler_name))?;

    match o {
        Some(o) => Ok(Response::ok(o).to_json()),
        None => Err(Error::not_found("不存在的订单")),
    }
}
