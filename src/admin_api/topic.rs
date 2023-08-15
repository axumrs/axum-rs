use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use serde::Serialize;

use crate::{
    db::{tag, topic, Paginate},
    form::topic as form,
    handler_helper::{get_conn, log_error},
    md, model, BotConfig, Error, ID64Response, JsonRespone, Response, Result,
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
        pin: frm.pin,
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

    push_to_bot(&conn, id, &state.cfg.bot).await?;

    Ok(Response::ok(ID64Response { id }).to_json())
}

async fn push_to_bot(conn: &sqlx::MySqlPool, id: u64, bot_cfg: &BotConfig) -> Result<()> {
    let tp = topic::detail2bot(conn, id).await?;
    if tp.is_none() {
        return Err(Error::not_found("不存在的文章"));
    }
    let tp = tp.unwrap();
    let mut data = HashMap::new();
    data.insert("title", tp.title);
    data.insert("subject_name", tp.name);
    data.insert(
        "url",
        format!("https://axum.rs/topic/{}/{}", tp.subject_slug, tp.slug),
    );

    let cli = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(Error::from)?;
    cli.post(&bot_cfg.full_webhook_url())
        .json(&data)
        .send()
        .await
        .map_err(Error::from)?;
    Ok(())
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

#[derive(Default, Serialize)]
pub struct Topic2Edit {
    pub id: u64,
    pub title: String,
    pub subject_id: u32,
    pub slug: String,
    pub summary: String,
    pub author: String,
    pub src: String,
    pub cover: String,
    pub md: String,
    pub try_readable: bool,
    pub tags: Vec<String>,
    pub pin: u8,
}

pub async fn find(
    Extension(state): Extension<Arc<model::State>>,
    Path(id): Path<u64>,
) -> Result<JsonRespone<Topic2Edit>> {
    let handler_name = "admin/topic/find";

    let conn = get_conn(&state);
    let t = topic::find2edit(&conn, id)
        .await
        .map_err(log_error(handler_name))?;

    match t {
        Some(t) => {
            let tag_list = topic::get_tags(&conn, t.id)
                .await
                .map_err(log_error(handler_name))?;
            let mut tags = Vec::with_capacity(tag_list.len());
            for t in tag_list {
                tags.push(t.name);
            }

            let tt = Topic2Edit {
                id: t.id,
                title: t.title,
                subject_id: t.subject_id,
                slug: t.slug,
                summary: t.summary,
                author: t.author,
                src: t.src,
                cover: t.cover,
                md: t.md,
                try_readable: t.try_readable,
                tags,
                pin: t.pin,
            };
            Ok(Response::ok(tt).to_json())
        }
        None => Err(Error::not_found("不存在的文章")),
    }
}

pub async fn edit(
    Extension(state): Extension<Arc<model::State>>,
    Json(frm): Json<form::Update>,
) -> Result<JsonRespone<ID64Response>> {
    let handler_name = "admin/topic/edit";

    let conn = get_conn(&state);

    let m = model::Topic {
        id: frm.id,
        title: frm.title,
        subject_id: frm.subject_id,
        slug: frm.slug,
        summary: frm.summary,
        author: frm.author,
        src: frm.src,
        try_readable: frm.try_readable,
        cover: frm.cover,
        dateline: chrono::Local::now(),
        pin: frm.pin,
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

    let id = topic::edit(&conn, &m, &c, Some(tag_ids))
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(ID64Response { id }).to_json())
}
