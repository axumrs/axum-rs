use axum::extract::{Path, Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, ArcAppState, Error, Result,
};

pub async fn top(
    State(state): State<ArcAppState>,
) -> Result<resp::JsonResp<Vec<model::subject::Subject>>> {
    let handler_name = "api/user/subject/top";
    let sql = format!(
        "SELECT {} FROM {} WHERE is_del=false ORDER BY pin DESC,id DESC LIMIT 6",
        &model::subject::Subject::fields(),
        &model::subject::Subject::table()
    );

    let p = get_pool(&state);

    let ls = sqlx::query_as(&sql)
        .fetch_all(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(ls))
}

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::PageQuery>,
) -> Result<resp::JsonResp<model::subject::SubjectPaginate>> {
    let handler_name = "api/user/subject/list";
    let p = get_pool(&state);
    let data = model::subject::Subject::list(
        &*p,
        &model::subject::SubjectListFilter {
            pq: model::subject::SubjectPaginateReq {
                page: frm.page(),
                page_size: frm.page_size(),
            },
            order: None,
            status: None,
            is_del: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn detail(
    State(state): State<ArcAppState>,
    Path(slug): Path<String>,
) -> Result<resp::JsonResp<resp::subject::Detail>> {
    let handler_name = "api/user/subject/topic_list";
    let p = get_pool(&state);

    let subject = model::subject::Subject::find(
        &*p,
        &model::subject::SubjectFindFilter {
            by: model::subject::SubjectFindBy::Slug(slug.clone()),
            is_del: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    let subject = match subject {
        Some(v) => v,
        None => return Err(Error::new("不存在的专题")),
    };

    let topic_list = service::topic::list_all_for_subject(&*p, slug)
        .await
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::subject::Detail {
        subject,
        topic_list,
    }))
}
