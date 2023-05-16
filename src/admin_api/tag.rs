use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};

use crate::{
    db::{tag, Paginate},
    form::tag as form,
    handler_helper::{get_conn, log_error},
    model, Error, IDResponse, JsonRespone, Response, Result,
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

pub async fn find(
    Extension(state): Extension<Arc<model::State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<model::Tag>> {
    let handler_name = "admin/tag/find";

    let conn = get_conn(&state);
    let t = tag::find(&conn, &model::TagFindBy::ID(id), None)
        .await
        .map_err(log_error(handler_name))?;
    match t {
        Some(t) => Ok(Response::ok(t).to_json()),
        None => Err(Error::not_found("不存在的标签")),
    }
}

pub async fn del(
    Extension(state): Extension<Arc<model::State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/tag/del";

    let conn = get_conn(&state);
    tag::del_or_restore(&conn, id, true)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn restore(
    Extension(state): Extension<Arc<model::State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/tag/restore";

    let conn = get_conn(&state);
    tag::del_or_restore(&conn, id, false)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn add(
    Extension(state): Extension<Arc<model::State>>,
    Json(frm): Json<form::Create>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/tag/add";

    let conn = get_conn(&state);
    let id = tag::add(
        &conn,
        &model::Tag {
            name: frm.name,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn edit(
    Extension(state): Extension<Arc<model::State>>,
    Json(frm): Json<form::Update>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/tag/edit";

    let conn = get_conn(&state);
    tag::edit(
        &conn,
        &model::Tag {
            id: frm.id,
            name: frm.name,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id: frm.id }).to_json())
}
