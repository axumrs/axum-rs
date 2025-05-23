use std::collections::HashMap;

use axum::extract::{Path, Query, State};

use crate::{
    api::{get_pool, log_error},
    form, mid, model, resp, service, ArcAppState, Error, Result,
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

pub async fn latest(
    State(state): State<ArcAppState>,
) -> Result<resp::JsonResp<Vec<model::subject::Subject>>> {
    let handler_name = "api/user/subject/top";
    let sql = format!(
        "SELECT {} FROM {} WHERE is_del=false ORDER BY id DESC LIMIT 6",
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
            name: None,
            slug: None,
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

pub async fn get_slug(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonResp<String>> {
    let handler_name = "api/user/subject/get_slug";
    let p = get_pool(&state);
    let subject = match model::subject::Subject::find(
        &*p,
        &model::subject::SubjectFindFilter {
            by: model::subject::SubjectFindBy::Id(id),
            is_del: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("不存在的专题")),
    };
    Ok(resp::ok(subject.slug))
}

#[derive(serde::Deserialize)]
pub struct Purchased {
    pub ids: String,
}
impl Purchased {
    pub fn ids(&self) -> Vec<&str> {
        self.ids.split(',').map(|s| s).collect()
    }
}
pub async fn purchased(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Query(frm): Query<Purchased>,
) -> Result<resp::JsonResp<HashMap<String, bool>>> {
    let handler_name = "api/user/subject/purchased";
    let user = match user_auth.user_opt() {
        Some(v) => v,
        None => return Ok(resp::ok(HashMap::new())),
    };

    let p = get_pool(&state);

    let data = service::order::purchased_services(&*p, &user.id, &frm.ids())
        .await
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(data))
}

pub async fn is_purchased(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Path(id): Path<String>,
) -> Result<resp::JsonResp<bool>> {
    let handler_name = "api/user/subject/is_purchased";
    let user = match user_auth.user_opt() {
        Some(v) => v,
        None => return Ok(resp::ok(false)),
    };
    let p = get_pool(&state);
    let data = service::order::is_a_purchased_service(&*p, &user.id, &id)
        .await
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(data))
}
