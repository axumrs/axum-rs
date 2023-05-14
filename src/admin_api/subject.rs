use std::sync::Arc;

use axum::{Extension, Json};

use crate::{
    db::subject,
    form::subject as form,
    handler_helper::{get_conn, log_error},
    model::{State, Subject},
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
            price: frm.price,
            status: frm.status.unwrap_or_default(),
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id }).to_json())
}
