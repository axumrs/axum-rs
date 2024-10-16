use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Local;
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::topic::ListForAdmin>,
) -> Result<resp::JsonResp<model::pagination::Paginate<model::topic_views::TopicSubjectWithTags>>> {
    let handler_name = "admin/topic/list";
    let p = get_pool(&state);

    let subject_id = if let Some(subject_name) = &frm.subject_name {
        let sql = format!(
            "SELECT id FROM {} WHERE name ILIKE $1",
            &model::subject::Subject::table()
        );
        let param = format!("%{}%", subject_name);
        let sb: Option<(String,)> = sqlx::query_as(&sql)
            .bind(&param)
            .fetch_optional(&*p)
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        match sb {
            Some(v) => Some(v.0),
            None => None,
        }
    } else {
        None
    };

    let data = service::topic::list_opt(
        &*p,
        &model::topic_views::VTopicSubjectListFilter {
            is_del: frm.is_del(),
            order: Some("id DESC".into()),
            title: frm.title,
            subject_id,
            slug: frm.slug,

            subject_slug: frm.subject_slug,
            subject_is_del: None,
            status: None,
            pq: model::topic_views::VTopicSubjectPaginateReq {
                page: frm.pq.page(),
                page_size: frm.pq.page_size(),
            },
            v_topic_subject_list_between_datelines: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn add(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::topic::Add>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "admin/topic/add";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let tag_names = frm
        .tag_names
        .iter()
        .map(|tn| tn.as_str())
        .collect::<Vec<_>>();

    let m = service::topic::add(
        &*p,
        model::topic::Topic {
            title: frm.title,
            subject_id: frm.subject_id,
            slug: frm.slug,
            summary: frm.summary,
            author: frm.author,
            src: frm.src,
            hit: 0,
            dateline: Local::now(),
            try_readable: frm.try_readable,
            is_del: false,
            cover: frm.cover,
            md: frm.md,
            pin: frm.pin,
            ..Default::default()
        },
        &tag_names,
        &state.cfg.topic_section_secret_key,
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::IDResp { id: m.id }))
}

pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::topic::Edit>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/topic/edit";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let tag_names = frm
        .base
        .tag_names
        .iter()
        .map(|tn| tn.as_str())
        .collect::<Vec<_>>();

    let p = get_pool(&state);

    let m = match model::topic::Topic::find(
        &*p,
        &model::topic::TopicFindFilter {
            id: Some(frm.id.clone()),
            subject_id: None,
            slug: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("不存在的文章")),
    };

    let aff = service::topic::edit(
        &*p,
        &model::topic::Topic {
            title: frm.base.title,
            subject_id: frm.base.subject_id,
            slug: frm.base.slug,
            summary: frm.base.summary,
            author: frm.base.author,
            src: frm.base.src,
            try_readable: frm.base.try_readable,
            cover: frm.base.cover,
            md: frm.base.md,
            pin: frm.base.pin,
            ..m
        },
        &tag_names,
        &state.cfg.topic_section_secret_key,
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn del(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/topic/del";
    let p = get_pool(&state);

    let aff = model::topic::Topic::update_is_del(&*p, &true, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn res(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/topic/res";
    let p = get_pool(&state);
    let aff = model::topic::Topic::update_is_del(&*p, &false, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}
