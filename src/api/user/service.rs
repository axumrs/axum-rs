use axum::extract::{Path, Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::service::ListForUser>,
) -> Result<resp::JsonResp<model::service::ServicePaginate>> {
    let handler_name = "user/service/list";
    let p = get_pool(&state);

    let data = model::service::Service::list(
        &*p,
        &model::service::ServiceListFilter {
            pq: model::service::ServicePaginateReq {
                page: frm.page(),
                page_size: frm.page_size(),
            },
            order: Some("is_subject ASC, pin DESC, id DESC".into()),
            name: None,
            is_subject: None,
            is_off: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn find_by_subject(
    State(state): State<ArcAppState>,
    Path(subject_id): Path<String>,
) -> Result<resp::JsonResp<model::service::Service>> {
    let handler_name = "user/service/find_by_subject";
    let p = get_pool(&state);

    let data = match model::service::Service::find(
        &*p,
        &model::service::ServiceFindFilter {
            id: None,
            is_subject: Some(true),
            target_id: Some(subject_id),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("该专题尚未上架")),
    };

    if data.is_off {
        return Err(Error::new("该服务已下架"));
    }

    Ok(resp::ok(data))
}
