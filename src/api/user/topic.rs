use axum::extract::{Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, ArcAppState, Error, Result,
};

pub async fn top(
    State(state): State<ArcAppState>,
) -> Result<resp::JsonResp<Vec<model::topic_views::TopicSubjectWithTags>>> {
    let handler_name = "api/user/topic/top";
    let p = get_pool(&state);

    let data = service::topic::list_all_opt(
        &*p,
        &model::topic_views::VTopicSubjectListAllFilter {
            limit: Some(10),
            order: Some("hit DESC, id DESC".into()),
            title: None,
            subject_id: None,
            slug: None,
            is_del: Some(false),
            subject_slug: None,
            subject_is_del: Some(false),
            status: None,
            v_topic_subject_list_all_between_datelines: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::PageQuery>,
) -> Result<resp::JsonResp<model::pagination::Paginate<model::topic_views::TopicSubjectWithTags>>> {
    let handler_name = "api/user/topic/list";
    let p = get_pool(&state);

    let data = service::topic::list_opt(
        &*p,
        &model::topic_views::VTopicSubjectListFilter {
            order: Some("id DESC".into()),
            title: None,
            subject_id: None,
            slug: None,
            is_del: Some(false),
            subject_slug: None,
            subject_is_del: Some(false),
            status: None,
            pq: model::topic_views::VTopicSubjectPaginateReq {
                page: frm.page(),
                page_size: frm.page_size(),
            },
            v_topic_subject_list_between_datelines: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}
