use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};

use crate::{
    db::{subject, Paginate},
    form::subject as form,
    handler_helper::{get_conn, log_error},
    model::{self, State, Subject},
    Error, IDResponse, JsonRespone, Response, Result,
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
            pin: frm.pin,
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

pub async fn del(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/subject/del";

    let conn = get_conn(&state);
    subject::del_or_restore(&conn, id, true)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id }).to_json())
}
pub async fn restore(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/subject/restore";

    let conn = get_conn(&state);
    subject::del_or_restore(&conn, id, false)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<model::Subject>> {
    let handler_name = "admin/subject/find";

    let conn = get_conn(&state);

    let s = subject::find(&conn, model::SubjectFindBy::ID(id), None)
        .await
        .map_err(log_error(handler_name))?;
    match s {
        Some(s) => Ok(Response::ok(s).to_json()),
        None => Err(Error::not_found("不存在的专题")),
    }
}

pub async fn update(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::Update>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/subject/update";

    let conn = get_conn(&state);
    let m = model::Subject {
        id: frm.id,
        name: frm.name,
        slug: frm.slug,
        summary: frm.summary,
        cover: frm.cover,
        price: frm.price * 100,
        status: frm.status,
        pin: frm.pin,
        ..Default::default()
    };

    subject::update(&conn, &m)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(IDResponse { id: frm.id }).to_json())
}
