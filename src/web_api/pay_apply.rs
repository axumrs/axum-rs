use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use validator::Validate;

use crate::{
    db::pay_apply,
    form::pay_apply as form,
    handler_helper::{get_conn, log_error},
    middleware::UserAuth,
    model::{self, State},
    Error, ID64Response, JsonRespone, Response, Result,
};

pub async fn add(
    Extension(state): Extension<Arc<State>>,
    UserAuth(claims): UserAuth,
    Json(frm): Json<form::Create>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "web/pay_apply/add";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let conn = get_conn(&state);

    let id = pay_apply::add(
        &conn,
        &model::PayApply {
            order_id: frm.order_id,
            price: frm.price,
            currency: frm.currency,
            types: frm.types,
            tx_id: frm.tx_id,
            img: frm.img,
            user_id: claims.id,
            dateline: chrono::Local::now(),
            status: model::PayApplyStatus::Pending,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(ID64Response { id }).to_json())
}

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    UserAuth(claims): UserAuth,
    Path(order_id): Path<u64>,
) -> Result<JsonRespone<model::PayApply>> {
    let handler_name = "web/pay_apply/find";

    let conn = get_conn(&state);
    let pa = pay_apply::find(
        &conn,
        &model::PayApplyFindBy::Owner {
            order_id,
            user_id: claims.id,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    match pa {
        Some(pa) => Ok(Response::ok(pa).to_json()),
        None => Err(Error::not_found("没有支付证明")),
    }
}
