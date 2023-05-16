use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};

use crate::{
    db::{tag, topic, Paginate},
    form::topic as form,
    handler_helper::{get_conn, log_error},
    md, model, ID64Response, JsonRespone, Response, Result,
};

pub async fn add(
    Extension(state): Extension<Arc<model::State>>,
    Json(frm): Json<form::Create>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "admin/topic/add";

    let conn = get_conn(&state);

    let m = model::Topic {
        title: frm.title,
        subject_id: frm.subject_id,
        slug: frm.slug,
        summary: frm.summary,
        author: frm.author,
        src: frm.src,
        try_readable: frm.try_readable,
        cover: frm.cover,
        dateline: chrono::Local::now(),
        ..Default::default()
    };
    let c = model::TopicContent {
        html: md::to_html(&frm.md),
        md: frm.md,
        ..Default::default()
    };

    let tag_ids = tag::auto(&conn, &frm.tags)
        .await
        .map_err(log_error(handler_name))?;

    let id = topic::add(&conn, &m, &c, Some(tag_ids))
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(ID64Response { id }).to_json())
}

pub async fn list(
    Extension(state): Extension<Arc<model::State>>,
    Query(frm): Query<form::List2Admin>,
) -> Result<JsonRespone<Paginate<model::Topic2AdminList>>> {
    let handler_name = "admin/topic/list";

    let conn = get_conn(&state);

    let p = topic::list2admin(
        &conn,
        &model::Topic2AdminListWith {
            title: frm.title,
            slug: frm.slug,
            subject_name: frm.subject_name,
            try_readable: frm.try_readable,
            is_del: frm.is_del,
            page: frm.page,
            page_size: frm.page_size,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}

pub async fn del(
    Extension(state): Extension<Arc<model::State>>,
    Path(id): Path<u64>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "admin/topic/del";

    let conn = get_conn(&state);
    topic::del_or_restore(&conn, id, true)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(ID64Response { id }).to_json())
}
pub async fn restore(
    Extension(state): Extension<Arc<model::State>>,
    Path(id): Path<u64>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "admin/topic/restore";

    let conn = get_conn(&state);
    topic::del_or_restore(&conn, id, false)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(ID64Response { id }).to_json())
}
pub async fn find(
    Extension(state): Extension<Arc<model::State>>,
    Path(id): Path<u64>,
) -> Result<JsonRespone<Option<model::Topic>>> {
    unimplemented!()
}
