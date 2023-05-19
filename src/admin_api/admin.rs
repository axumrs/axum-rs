use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};

use crate::{
    db::{admin, Paginate},
    form::{admin as form, PaginateForm},
    handler_helper::{get_conn, log_error},
    model::{self, State},
    Error, IDResponse, JsonRespone, Response, Result,
};

pub async fn add(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::Create>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/admin/add";

    let conn = get_conn(&state);
    let m = model::Admin {
        username: frm.username,
        password: frm.password,
        ..Default::default()
    };
    let id = admin::add(&conn, &m)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<PaginateForm>,
) -> Result<JsonRespone<Paginate<model::Admin>>> {
    let handler_name = "admin/admin/list";

    let conn = get_conn(&state);
    let p = admin::list(
        &conn,
        &model::PaginateWith {
            page: frm.page,
            page_size: frm.page_size,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<model::Admin>> {
    let handler_name = "admin/admin/find";

    let conn = get_conn(&state);
    let a = admin::find(&conn, &model::AdminFindBy::ID(id))
        .await
        .map_err(log_error(handler_name))?;

    match a {
        Some(a) => Ok(Response::ok(a).to_json()),
        None => Err(Error::not_found("不存在的管理员")),
    }
}

pub async fn edit(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::Update>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/admin/edit";

    let conn = get_conn(&state);
    admin::edit(
        &conn,
        &model::Admin2Edit {
            id: frm.id,
            username: frm.username,
            password: frm.password,
        },
    )
    .await
    .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id: frm.id }).to_json())
}

pub async fn del(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/admin/del";

    let conn = get_conn(&state);
    admin::del_or_restore(&conn, id, true)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn restore(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/admin/restore";

    let conn = get_conn(&state);
    admin::del_or_restore(&conn, id, false)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id }).to_json())
}
