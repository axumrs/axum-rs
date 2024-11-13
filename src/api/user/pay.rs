use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    api::{get_pool, log_error},
    form, mid, model, resp, service, ArcAppState, Result,
};

/// 支付
pub async fn add(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Json(frm): Json<form::pay::UserPay>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "user/pay/add";
    let user = user_auth.user().map_err(log_error(handler_name))?;
    let p = get_pool(&state);

    let m = model::pay::Pay {
        order_id: frm.order_id,
        user_id: user.id.clone(),
        amount: frm.amount,
        currency: frm.currency,
        tx_id: frm.tx_id,
        method: frm.method,
        status: model::pay::Status::Pending,
        is_via_admin: false,
        ..Default::default()
    };

    let m = service::pay::create(&*p, m, &state.cfg.currency, frm.re_pay)
        .await
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::IDResp { id: m.id }))
}

/// 完成支付
pub async fn complete(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Json(frm): Json<form::pay::UserConfirm>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "user/pay/complete";
    let user = user_auth.user().map_err(log_error(handler_name))?;
    let p = get_pool(&state);

    let aff = service::pay::complete(
        &*p,
        frm.pay_id,
        frm.order_id,
        Some(user.id.clone()),
        &state.cfg,
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}

/// 支付详情
pub async fn detail_by_order(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Path(order_id): Path<String>,
) -> Result<resp::JsonResp<Option<model::pay::Pay>>> {
    let handler_name = "user/pay/detail";
    let user = user_auth.user().map_err(log_error(handler_name))?;
    let p = get_pool(&state);
    let pay = service::pay::find(
        &*p,
        &model::pay::PayFindFilter {
            id: None,
            user_id: Some(user.id.clone()),
            order_id: Some(order_id),
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(pay))
}
