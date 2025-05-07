use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Local;
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, mid, model, resp, utils, ArcAppState, Error, Result,
};

pub async fn create(
    State(state): State<ArcAppState>,
    _: mid::AdminAuth,
    Json(frm): Json<form::promotion::Add>,
) -> Result<resp::JsonResp<String>> {
    let hanlder_name = "admin/promotion/create";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(hanlder_name))?;

    let id = utils::id::new();
    let p = get_pool(&state);

    let m = model::promotion::Promotion {
        id,
        name: frm.inner.name,
        content: frm.inner.content,
        url: frm.inner.url,
        img: frm.inner.img,
        dateline: Local::now(),
    };

    m.insert(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(hanlder_name))?;
    Ok(resp::ok(m.id))
}

pub async fn edit(
    State(state): State<ArcAppState>,
    _: mid::AdminAuth,
    Json(frm): Json<form::promotion::Edit>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/promotion/edit";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let m = match model::promotion::Promotion::find(
        &mut *tx,
        &model::promotion::PromotionFindFilter {
            id: Some(frm.id.clone()),
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("不存在的推广")),
        },
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into());
        }
    };

    let m = model::promotion::Promotion {
        name: frm.inner.name,
        content: frm.inner.content,
        url: frm.inner.url,
        img: frm.inner.img,
        ..m
    };

    let aff = match m.update(&mut *tx).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into());
        }
    };

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn del(
    State(state): State<ArcAppState>,
    _: mid::AdminAuth,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/promotion/del";
    let p = get_pool(&state);
    let aff = model::promotion::Promotion::real_del(&*p, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn list(
    State(state): State<ArcAppState>,
    _: mid::AdminAuth,
    Query(frm): Query<form::promotion::ListForAdmin>,
) -> Result<resp::JsonResp<model::promotion::PromotionPaginate>> {
    let handler_name = "admin/promotion/list";
    let p = get_pool(&state);
    let data = model::promotion::Promotion::list(
        &*p,
        &model::promotion::PromotionListFilter {
            name: frm.name,
            pq: model::promotion::PromotionPaginateReq {
                page: frm.pq.page(),
                page_size: frm.pq.page_size(),
            },
            order: Some("id DESC".into()),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(data))
}
