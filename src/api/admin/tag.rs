use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, ArcAppState, Error, Result,
};

pub async fn all(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::ListAll>,
) -> Result<resp::JsonResp<Vec<model::tag::Tag>>> {
    let handler_name = "admin/tag/all";
    let p = get_pool(&state);
    let data = model::tag::Tag::list_all(
        &*p,
        &model::tag::TagListAllFilter {
            limit: frm.limit,
            order: Some("id ASC".into()),
            name: None,
            is_del: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(data))
}

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::tag::ListForAdmin>,
) -> Result<resp::JsonResp<model::tag::TagPaginate>> {
    let handler_name = "admin/tag/list";
    let p = get_pool(&state);
    let data = model::tag::Tag::list(
        &*p,
        &model::tag::TagListFilter {
            pq: model::tag::TagPaginateReq {
                page: frm.pq.page(),
                page_size: frm.pq.page_size(),
            },
            order: None,
            is_del: frm.is_del(),
            name: frm.name,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn real_del(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/tag/del";
    let p = get_pool(&state);
    let aff = model::tag::Tag::real_del(&*p, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn add(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::tag::Add>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "admin/tag/add";
    let p = get_pool(&state);
    let m = service::tag::add(
        &*p,
        model::tag::Tag {
            name: frm.name,
            ..Default::default()
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::IDResp { id: m.id }))
}

pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::tag::Edit>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/tag/edit";
    let p = get_pool(&state);
    let m = match model::tag::Tag::find(
        &*p,
        &model::tag::TagFindFilter {
            id: Some(frm.id.clone()),
            name: None,
            is_del: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("不存在的标签")),
    };

    let aff = service::tag::edit(
        &*p,
        &model::tag::Tag {
            name: frm.base.name,
            ..m
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}
