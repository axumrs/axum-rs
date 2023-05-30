use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use validator::Validate;

use crate::{
    db::{order, Paginate},
    form::order as form,
    handler_helper::{get_conn, log_error},
    middleware::UserAuth,
    model::{self, State},
    Error, ID64Response, JsonRespone, Response, Result,
};

pub async fn create(
    Extension(state): Extension<Arc<State>>,
    UserAuth(claims): UserAuth,
    Json(frm): Json<form::Create>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "web/order/create";

    let conn = get_conn(&state);

    let m = model::Order::new(claims.id, frm.price).map_err(log_error(handler_name))?;
    let s = model::OrderSnap {
        snap: frm.snap,
        ..Default::default()
    };
    let id = order::add(&conn, &m, &s)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(ID64Response { id }).to_json())
}

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    UserAuth(claims): UserAuth,
    Path(id): Path<u64>,
) -> Result<JsonRespone<model::OrderFull>> {
    let handler_name = "web/order/find";

    let conn = get_conn(&state);

    let o = order::find(&conn, id, Some(claims.id), None)
        .await
        .map_err(log_error(handler_name))?;

    match o {
        Some(o) => Ok(Response::ok(o).to_json()),
        None => Err(Error::not_found("不存在的订单")),
    }
}

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    UserAuth(claims): UserAuth,
    Query(frm): Query<crate::form::PaginateForm>,
) -> Result<JsonRespone<Paginate<model::OrderWithUser>>> {
    let handler_name = "web/order/list";

    let conn = get_conn(&state);

    let p = order::list(
        &conn,
        &model::OrderListWith {
            user_id: Some(claims.id),
            is_del: Some(false),
            page: frm.page,
            page_size: frm.page_size,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}

pub async fn pay(
    Extension(state): Extension<Arc<State>>,
    UserAuth(claims): UserAuth,
    Json(frm): Json<crate::form::pay::Create>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "web/order/pay";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let conn = get_conn(&state);

    order::update_with_pay(
        &conn,
        frm.order_id,
        claims.id,
        &model::Pay {
            order_id: frm.order_id,
            user_id: claims.id,
            price: frm.price,
            currency: frm.currency,
            types: frm.types,
            tx_id: frm.tx_id,
            status: frm.status,
            dateline: chrono::Local::now(),
            ..Default::default()
        },
        None,
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(ID64Response { id: frm.order_id }).to_json())
}
