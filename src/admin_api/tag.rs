use std::sync::Arc;

use axum::{extract::Query, Extension};

use crate::{
    db::{tag, Paginate},
    form::tag as form,
    handler_helper::{get_conn, log_error},
    model, JsonRespone, Response, Result,
};

pub async fn list(
    Extension(state): Extension<Arc<model::State>>,
    Query(frm): Query<form::List>,
) -> Result<JsonRespone<Paginate<model::Tag>>> {
    let handler_name = "admin/tag/list";

    let conn = get_conn(&state);
    let p = tag::list(
        &conn,
        &model::TagListWith {
            name: frm.name,
            is_del: frm.is_del,
            page: frm.page,
            page_size: frm.page_size,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}
