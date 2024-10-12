use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::subject::ListForAdmin>,
) -> Result<resp::JsonResp<model::subject::SubjectPaginate>> {
    let handler_name = "admin/subject/list";
    let p = get_pool(&state);
    let subjects = model::subject::Subject::list(
        &*p,
        &model::subject::SubjectListFilter {
            pq: model::subject::SubjectPaginateReq {
                page: frm.pq.page(),
                page_size: frm.pq.page_size(),
            },
            order: None,
            is_del: frm.is_del(),
            status: frm.status,
            name: frm.name,
            slug: frm.slug,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(subjects))
}

pub async fn add(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::subject::Add>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "admin/subject/add";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);
    let m = service::subject::add(
        &*p,
        model::subject::Subject {
            name: frm.name,
            slug: frm.slug,
            summary: frm.summary,
            is_del: false,
            cover: frm.cover,
            status: frm.status,
            price: frm.price,
            pin: frm.pin,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::IDResp { id: m.id }))
}

pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::subject::Edit>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/subject/edit";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);
    let aff = service::subject::edit(
        &*p,
        &model::subject::Subject {
            id: frm.id,
            name: frm.base.name,
            slug: frm.base.slug,
            summary: frm.base.summary,
            cover: frm.base.cover,
            status: frm.base.status,
            price: frm.base.price,
            pin: frm.base.pin,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn del(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
    Query(frm): Query<form::subject::RealDel>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/subject/del";
    let p = get_pool(&state);
    let real = match frm.real {
        Some(v) => v,
        None => false,
    };
    let aff = if real {
        model::subject::Subject::real_del(&*p, &id).await
    } else {
        model::subject::Subject::update_is_del(&*p, &true, &id).await
    }
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn res(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/subject/res";
    let p = get_pool(&state);
    let aff = model::subject::Subject::update_is_del(&*p, &false, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}
