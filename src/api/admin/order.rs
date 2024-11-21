use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::{Postgres, QueryBuilder};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, ArcAppState, Error, Result,
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
    Ok(resp::ok(resp::IDResp { id: "".to_string() }))
}
