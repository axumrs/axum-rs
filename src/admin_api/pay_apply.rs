use std::sync::Arc;

use axum::{extract::Path, Extension, Json};

use crate::{
    db::pay_apply,
    form::pay_apply as form,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    Error, ID64Response, JsonRespone, Response, Result,
};

pub async fn find(
    Extension(state): Extension<Arc<State>>,

    Path((order_id, user_id)): Path<(u64, u32)>,
) -> Result<JsonRespone<model::PayApply>> {
    let handler_name = "admin/pay_apply/find";

    let conn = get_conn(&state);
    let pa = pay_apply::find(&conn, &model::PayApplyFindBy::Owner { order_id, user_id })
        .await
        .map_err(log_error(handler_name))?;

    match pa {
        Some(pa) => Ok(Response::ok(pa).to_json()),
        None => Err(Error::not_found("没有支付证明")),
    }
}

pub async fn reject(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::AdminReject>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "admin/pay_apply/reject";
    let conn = get_conn(&state);
    pay_apply::reject(&conn, frm.id, &frm.reason)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(ID64Response { id: frm.id }).to_json())
}

pub async fn accept(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::AdminAccept>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "admin/pay_apply/accept";
    let conn = get_conn(&state);
    let pa = pay_apply::find(&conn, &model::PayApplyFindBy::ID(frm.id))
        .await
        .map_err(log_error(handler_name))?;
    if pa.is_none() {
        return Err(Error::not_found("不存在的支付证明"));
    }
    let pa = pa.unwrap();
    let pay = model::Pay {
        order_id: (&pa).order_id,
        user_id: (&pa).user_id,
        price: (&pa).price,
        currency: (&pa).currency.clone(),
        types: (&pa).types.clone(),
        tx_id: (&pa).tx_id.clone(),
        status: model::PayStatus::Finished,
        dateline: (&pa).dateline.clone(),
        ..Default::default()
    };

    pay_apply::accept(&conn, &pa, &pay)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(ID64Response { id: frm.id }).to_json())
}
