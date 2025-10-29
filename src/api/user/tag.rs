use axum::extract::{Path, Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::PageQuery>,
) -> Result<resp::JsonResp<model::pagination::Paginate<model::tag::TagWithTopicCount>>> {
    let handler_name = "user/tag/list";
    let p = get_pool(&state);
    let data = service::tag::list_with_topic_count(&*p, frm.page(), frm.page_size())
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn detail(
    State(state): State<ArcAppState>,
    Path(name): Path<String>,
    Query(frm): Query<form::PageQuery>,
) -> Result<resp::JsonResp<model::tag::TagWithTopicListAndCount>> {
    let handler_name = "user/tag/detail";
    let p = get_pool(&state);
    let f = model::tag::TagFindFilter {
        id: None,
        name: Some(name),
        is_del: Some(false),
    };
    let tag_with_topic_count = match service::tag::find_with_topic_count(&*p, Some(&f), None)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("不存在该标签").into()),
    };

    let tp = model::topic_tag::TopicTag::list(
        &*p,
        &model::topic_tag::TopicTagListFilter {
            pq: model::topic_tag::TopicTagPaginateReq {
                page: frm.page(),
                page_size: frm.page_size(),
            },
            order: None,
            topic_id: None,
            tag_id: Some(tag_with_topic_count.tag.id.clone()),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    let mut r = Vec::with_capacity(tp.data.len());

    for t in tp.data {
        let f = model::topic_views::VTopicSubjectFindFilter {
            id: Some(t.topic_id.clone()),
            subject_id: None,
            slug: None,
            is_del: Some(false),
            subject_slug: None,
            subject_is_del: Some(false),
        };
        let tf = model::topic_tag::VTopicTagWithTagListAllFilter {
            limit: None,
            order: None,
            topic_id: t.topic_id,
            name: None,
            is_del: Some(false),
        };
        let m = service::topic::find_opt(&*p, Some(&f), &tf, None, true)
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        if let Some(m) = m {
            r.push(m);
        }
    }

    Ok(resp::ok(model::tag::TagWithTopicListAndCount {
        tag_with_topic_count,
        topic_paginate: model::pagination::Paginate {
            total: tp.total,
            total_page: tp.total_page,
            page: tp.page,
            page_size: tp.page_size,
            data: r,
        },
    }))
}
