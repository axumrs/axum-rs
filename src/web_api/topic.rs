use std::sync::Arc;

use axum::{extract::Query, Extension};

use crate::{
    db::{topic, Paginate},
    form::topic as form,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    JsonRespone, Response, Result,
};

pub async fn top10(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonRespone<Vec<model::Topic2WebList>>> {
    let handler_name = "web/topic/top10";

    let conn = get_conn(&state);
    let p = topic::list2web(
        &conn,
        &model::Topic2WebListWith {
            order_by_hit: true,
            page: 0,
            page_size: 10,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p.data).to_json())
}
pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<form::List2Web>,
) -> Result<JsonRespone<Paginate<model::Topic2WebList>>> {
    let handler_name = "web/topic/list";

    let conn = get_conn(&state);
    let p = topic::list2web(
        &conn,
        &model::Topic2WebListWith {
            subject_name: frm.subject_name,
            subject_slug: frm.subject_slug,
            page: frm.page,
            page_size: frm.page_size,
            order_by_hit: frm.order_by_hit.unwrap_or(false),
            title: frm.title,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}
