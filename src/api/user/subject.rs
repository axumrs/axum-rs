use axum::extract::{Path, Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, ArcAppState, Error, Result,
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

// pub async fn topic_list(
//     State(state): State<ArcAppState>,
//     Path(slug): Path<String>,
// ) -> Result<resp::JsonResp<Vec<model::topic::Topic>>> {
//     unimplemented!()
// }
