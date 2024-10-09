use axum::extract::{Path, Query, State};

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

pub async fn detail(
    State(state): State<ArcAppState>,
    Path((subject_slug, slug)): Path<(String, String)>,
) -> Result<resp::JsonResp<model::topic_views::TopicSubjectWithTagsAndProctedSections>> {
    let handler_name = "api/user/topic/detail";
    let p = get_pool(&state);

    let data = service::topic::find_detail(&*p, &slug, &subject_slug)
        .await
        .map_err(log_error(handler_name))?;

    // 是否需要内容保护
    let need_procted = true;
    if need_procted {
        let (secs, protected_ids) =
            service::topic::gen_protected_content(&*p, data.sections, &state.cfg.protected_content)
                .await
                .map_err(log_error(handler_name))?;
        let data = model::topic_views::TopicSubjectWithTagsAndSections {
            sections: secs,
            ..data
        };
        return Ok(resp::ok(
            model::topic_views::TopicSubjectWithTagsAndProctedSections {
                topic_subject_with_tags_and_sections: data,
                protected: model::topic_views::TopicProctedMeta {
                    ids: protected_ids,
                    catpcha: state.cfg.protected_content.guest_captcha.clone(),
                },
            },
        ));
    }
    Ok(resp::ok(
        model::topic_views::TopicSubjectWithTagsAndProctedSections {
            topic_subject_with_tags_and_sections: data,
            protected: model::topic_views::TopicProctedMeta::default(),
        },
    ))
}
