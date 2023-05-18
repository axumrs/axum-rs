use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};

use crate::{
    db::{tag, Paginate},
    form::PaginateForm,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    Error, JsonRespone, Response, Result,
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

pub async fn detail(
    Extension(state): Extension<Arc<State>>,
    Path(name): Path<String>,
) -> Result<JsonRespone<model::Tag>> {
    let handler_name = "web/tag/detail";

    let conn = get_conn(&state);
    let p = tag::find(&conn, &model::TagFindBy::ExactName(&name), Some(false))
        .await
        .map_err(log_error(handler_name))?;
    match p {
        Some(p) => Ok(Response::ok(p).to_json()),
        None => Err(Error::not_found("不存在的主题")),
    }
}
