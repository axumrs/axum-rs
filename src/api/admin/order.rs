use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Local;
use rust_decimal::Decimal;
use sqlx::{Postgres, QueryBuilder};
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, utils, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::order::ListForAdmin>,
) -> Result<resp::JsonResp<model::pagination::Paginate<model::order::OrderWithUser>>> {
    let handler_name = "admin/order/list";
    let p = get_pool(&state);

    let q = QueryBuilder::new(
        r#"SELECT id, user_id, amount, actual_amount, status, "snapshot", allow_pointer, dateline, email, nickname FROM v_order_users WHERE 1=1"#,
    );
    let mut q = build_list_query(q, &frm);
    q.push(" ORDER BY id DESC")
        .push(" LIMIT ")
        .push_bind(frm.pq.page_size_to_bind())
        .push(" OFFSET ")
        .push_bind(frm.pq.offset_to_bind());

    let qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM v_order_users WHERE 1=1"#);
    let mut qc = build_list_query(qc, &frm);

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let rows = q
        .build_query_as()
        .fetch_all(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let data = model::pagination::Paginate::quick(count, frm.pq.page(), frm.pq.page_size(), rows);

    Ok(resp::ok(data))
}

fn build_list_query<'a>(
    mut q: QueryBuilder<'a, Postgres>,
    frm: &form::order::ListForAdmin,
) -> QueryBuilder<'a, Postgres> {
    q
}

#[derive(serde::Serialize)]
pub struct FindPayResp {
    pub has_pay: bool,
    pub pay: Option<model::pay::Pay>,
}
pub async fn find_pay(
    State(state): State<ArcAppState>,
    Path(order_id): Path<String>,
) -> Result<resp::JsonResp<FindPayResp>> {
    let handler_name = "admin/order/find_pay";
    let p = get_pool(&state);
    let pay = model::pay::Pay::find(
        &*p,
        &model::pay::PayFindFilter {
            id: None,
            order_id: Some(order_id),
            user_id: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(FindPayResp {
        has_pay: pay.is_some(),
        pay,
    }))
}

pub async fn add(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::order::AddForAdmin>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "admin/order/add";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let now = Local::now();

    let p = get_pool(&state);
    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 订单
    let order_id = utils::id::new();
    let snapshot = serde_json::json!(&frm.snap).to_string();

    let order = model::order::Order {
        id: order_id.clone(),
        user_id: frm.user_id.clone(),
        amount: frm.amount,
        actual_amount: frm.amount,
        status: model::order::Status::Pending,
        snapshot,
        allow_pointer: false,
        dateline: now,
    };

    if let Err(e) = order.insert(&mut *tx).await {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into());
    }

    // 支付
    let pay_id = utils::id::new();
    let pay = model::pay::Pay {
        id: pay_id,
        order_id: order_id.clone(),
        user_id: frm.user_id,
        amount: frm.amount,
        currency: frm.currency,
        tx_id: frm.tx_id,
        method: frm.method,
        status: model::pay::Status::Pending,
        is_via_admin: frm.is_via_admin,
        approved_time: now,
        approved_opinion: frm.approved_opinion,
        proof: frm.proof,
        dateline: now,
    };

    if let Err(e) = pay.insert(&mut *tx).await {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into());
    }

    // 完成订单
    if let Err(e) = service::pay::complete(&mut tx, pay.id, order_id, &state.cfg, None, true).await
    {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into());
    }

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::IDResp { id: "".to_string() }))
}
