use axum::extract::{Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::PageQuery>,
) -> Result<resp::JsonResp<model::subject::SubjectPaginate>> {
    let handler_name = "admin/subject/list";
    let p = get_pool(&state);
    let subjects = model::subject::Subject::list(
        &*p,
        &model::subject::SubjectListFilter {
            pq: model::subject::SubjectPaginateReq {
                page: frm.page(),
                page_size: frm.page_size(),
            },
            order: None,
            is_del: None,
            status: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(subjects))
}
