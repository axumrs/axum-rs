use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Local;
use rust_decimal::Decimal;
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, mid, model, resp, service, utils, ArcAppState, Error, Result,
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

pub async fn latest(
    State(state): State<ArcAppState>,
) -> Result<resp::JsonResp<Vec<model::topic_views::TopicSubjectWithTags>>> {
    let handler_name = "api/user/topic/top";
    let p = get_pool(&state);

    let data = service::topic::list_all_opt(
        &*p,
        &model::topic_views::VTopicSubjectListAllFilter {
            limit: Some(10),
            order: Some("id DESC".into()),
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
    user_auth: mid::UserAuth,
    Path((subject_slug, slug)): Path<(String, String)>,
) -> Result<resp::JsonResp<model::topic_views::TopicSubjectWithTagsAndProctedSectionsAndNeedLogin>>
{
    let handler_name = "api/user/topic/detail";
    let p = get_pool(&state);

    let data = service::topic::find_detail(&*p, &slug, &subject_slug)
        .await
        .map_err(log_error(handler_name))?;

    // 是否购买
    let is_purchased = if data.topic_subject_with_tags.topic_subjects.price > Decimal::ZERO {
        match user_auth.user_opt() {
            Some(u) => service::order::is_a_purchased_service(
                &*p,
                &u.id,
                &data.topic_subject_with_tags.topic_subjects.subject_id,
            )
            .await
            .map_err(log_error(handler_name))?,
            None => false,
        }
    } else {
        true
    };
    let is_need_buy = if data.topic_subject_with_tags.topic_subjects.try_readable {
        false
    } else {
        !is_purchased
    };

    // 是否登录
    if user_auth.token_opt().is_none() && !is_need_buy {
        let secs = service::topic::gen_guess_content(
            data.sections,
            &state.cfg.protected_content,
            is_need_buy,
        )
        .await
        .map_err(log_error(handler_name))?;
        let data = model::topic_views::TopicSubjectWithTagsAndSections {
            sections: secs,
            ..data
        };

        return Ok(resp::ok(
            model::topic_views::TopicSubjectWithTagsAndProctedSectionsAndNeedLogin {
                topic_subject_with_tags_and_procted_sections:
                    model::topic_views::TopicSubjectWithTagsAndProctedSections {
                        topic_subject_with_tags_and_sections: data,
                        protected: model::topic_views::TopicProctedMeta::default(),
                        need_buy: is_need_buy,
                    },
                need_login: true,
            },
        ));
    }

    if is_need_buy {
        return Ok(resp::ok(
            model::topic_views::TopicSubjectWithTagsAndProctedSectionsAndNeedLogin {
                topic_subject_with_tags_and_procted_sections:
                    model::topic_views::TopicSubjectWithTagsAndProctedSections {
                        topic_subject_with_tags_and_sections:
                            model::topic_views::TopicSubjectWithTagsAndSections {
                                sections: vec![],
                                ..data
                            },
                        protected: model::topic_views::TopicProctedMeta::default(),
                        need_buy: is_need_buy,
                    },
                need_login: false,
            },
        ));
    }

    if let Some(u) = user_auth.user_opt() {
        // 阅读历史
        let rh = Arc::new(model::read_history::ReadHistorie {
            id: utils::id::new(),
            user_id: u.id.clone(),
            subject_slug: data
                .topic_subject_with_tags
                .topic_subjects
                .subject_slug
                .clone(),
            slug: data.topic_subject_with_tags.topic_subjects.slug.clone(),
            subject_name: data.topic_subject_with_tags.topic_subjects.name.clone(),
            topic_title: data.topic_subject_with_tags.topic_subjects.title.clone(),
            dateline: Local::now(),
        });
        tokio::spawn(read_history(p.clone(), rh));
    }

    // 是否需要内容保护
    let need_procted = match user_auth.user_opt() {
        Some(u) => {
            // 是否订阅用户
            let is_subscribed = match &u.kind {
                &model::user::Kind::Normal => false,
                &model::user::Kind::Subscriber | &model::user::Kind::YearlySubscriber => true,
            };
            if !is_subscribed {
                // // 是否购买专题
                // let is_purchased = match user_auth.user_opt() {
                //     Some(u) => service::order::is_a_purchased_service(
                //         &*p,
                //         &u.id,
                //         &data.topic_subject_with_tags.topic_subjects.subject_id,
                //     )
                //     .await
                //     .map_err(log_error(handler_name))?,
                //     None => false,
                // };
                // tracing::debug!(
                //     "普通用户，是否购买：{is_purchased}, 是否需要内容保护：{}",
                //     !is_purchased
                // );
                // !is_purchased
                true
            } else {
                // tracing::debug!("订阅用户，不需要内容保护");
                false
            }
        }
        None => {
            // tracing::debug!("未登录用户，需要内容保护");
            true
        }
    };
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
            model::topic_views::TopicSubjectWithTagsAndProctedSectionsAndNeedLogin {
                topic_subject_with_tags_and_procted_sections:
                    model::topic_views::TopicSubjectWithTagsAndProctedSections {
                        topic_subject_with_tags_and_sections: data,
                        protected: model::topic_views::TopicProctedMeta {
                            ids: protected_ids,
                            catpcha: state.cfg.protected_content.guest_captcha.clone(),
                        },
                        need_buy: false,
                    },
                need_login: false,
            },
        ));
    }
    Ok(resp::ok(
        model::topic_views::TopicSubjectWithTagsAndProctedSectionsAndNeedLogin {
            topic_subject_with_tags_and_procted_sections:
                model::topic_views::TopicSubjectWithTagsAndProctedSections {
                    topic_subject_with_tags_and_sections: data,
                    protected: model::topic_views::TopicProctedMeta::default(),
                    need_buy: false,
                },
            need_login: false,
        },
    ))
}

pub async fn get_protected_content(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Json(frm): Json<form::topic::GetProtectedContent>,
) -> Result<resp::JsonResp<Vec<model::protected_content::ProtectedContent>>> {
    let handler_name = "api/user/topic/get_protected_content";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    user_auth.user().map_err(log_error(handler_name))?;

    let p = get_pool(&state);
    let secs = service::topic::get_protected_content(&*p, &frm.ids)
        .await
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(secs))
}

async fn read_history(
    p: Arc<sqlx::PgPool>,
    m: Arc<model::read_history::ReadHistorie>,
) -> Result<String> {
    let dateline = m.dateline - chrono::Duration::seconds(60);
    let count:(i64,) = sqlx::query_as("SELECT count(*) FROM read_histories WHERE user_id = $1 AND subject_slug = $2 AND slug = $3 AND dateline >= $4").bind(&m.user_id).bind(&m.subject_slug).bind(&m.slug).bind(&dateline).fetch_one(&*p).await?;
    if count.0 > 0 {
        return Ok(m.id.clone());
    }
    m.insert(&*p).await.map_err(Error::from)
}

pub async fn nav_page(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonResp<model::topic_page::NavPage>> {
    let handler_name = "/api/user/topic/nav_page";
    let p = get_pool(&state);
    let m = model::topic_page::NavPage::find(&*p, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    let m = match m {
        Some(v) => v,
        None => return Err(Error::new("不存在的文章")),
    };
    Ok(resp::ok(m))
}

pub async fn search(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::topic::Search>,
) -> Result<resp::JsonResp<Vec<model::topic_views::TopicSubjectForSearch>>> {
    let handler_name = "/api/user/topic/search";
    let p = get_pool(&state);
    let sql = r#"
    WITH search_topics AS (
        SELECT id, TS_RANK(search_vector, WEBSEARCH_TO_TSQUERY('chinese', $1)) AS rank FROM topics WHERE search_vector @@ WEBSEARCH_TO_TSQUERY('chinese', $1)
    )

    SELECT ts."id", "title", "subject_id", "slug", "summary", "try_readable", "name", "subject_slug" FROM v_topic_subjects AS ts INNER JOIN search_topics AS s ON s.id=ts.id ORDER BY rank DESC LIMIT 30
    "#;

    let data = sqlx::query_as(sql)
        .bind(&frm.keyword)
        .fetch_all(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}
