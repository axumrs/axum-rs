use std::sync::Arc;

use axum::{extract::Query, Extension};

use crate::{
    db::{subject, Paginate},
    form::subject as form,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    JsonRespone, Response, Result,
};

pub async fn top4(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonRespone<Vec<model::Subject>>> {
    let handler_name = "web/subject/top4";

    let conn = get_conn(&state);
    let p = subject::list(
        &conn,
        model::SubjectListWith {
            is_del: Some(false),
            page: 0,
            page_size: 4,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p.data).to_json())
}

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<form::List>,
) -> Result<JsonRespone<Paginate<model::Subject>>> {
    let handler_name = "web/subject/list";

    let conn = get_conn(&state);
    let p = subject::list(
        &conn,
        model::SubjectListWith {
            page: frm.page,
            page_size: frm.page_size,
            is_del: Some(false),
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}
