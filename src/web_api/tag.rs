use std::sync::Arc;

use axum::{extract::Query, Extension};

use crate::{
    db::{tag, Paginate},
    form::{tag as form, PaginateForm},
    handler_helper::{get_conn, log_error},
    model::{self, State},
    JsonRespone, Response, Result,
};

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<PaginateForm>,
) -> Result<JsonRespone<Paginate<model::Tag2WebList>>> {
    let handler_name = "web/tag/list";

    let conn = get_conn(&state);
    let p = tag::list2web(
        &conn,
        &model::Tag2WebListWith {
            page: frm.page,
            page_size: frm.page_size,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}
