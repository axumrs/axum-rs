use std::sync::Arc;

use axum::{extract::Query, Extension, Json};

use crate::{
    db::{subject, Paginate},
    form::subject as form,
    handler_helper::{get_conn, log_error},
    model::{self, State, Subject},
    IDResponse, JsonRespone, Response, Result,
};

pub async fn add(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::Create>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/subject/add";

    let conn = get_conn(&state);
    let id = subject::add(
        &conn,
        &Subject {
            name: frm.name,
            slug: frm.slug,
            summary: frm.summary,
            cover: frm.cover,
            price: frm.price * 100,
            status: frm.status.unwrap_or_default(),
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<form::List>,
) -> Result<JsonRespone<Paginate<model::Subject>>> {
    let handler_name = "admin/subject/list";

    let conn = get_conn(&state);
    let p = subject::list(
        &conn,
        model::SubjectListWith {
            page: frm.page,
            page_size: frm.page_size,
            name: frm.name,
            slug: frm.slug,
            status: frm.status,
            is_del: frm.is_del,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;
    Ok(Response::ok(p).to_json())
}
