use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Local;
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, model, resp, utils, ArcAppState, Error, Result,
};

pub async fn add(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::announcement::Add>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "admin/announcement/add";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let id = utils::id::new();
    let m = model::announcement::Announcement {
        id,
        title: frm.title,
        content: frm.content,
        dateline: Local::now(),
        hit: 0,
    };

    m.insert(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::IDResp { id: m.id }))
}

pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::announcement::Edit>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/announcement/edit";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);
    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let m = match model::announcement::Announcement::find(
        &mut *tx,
        &model::announcement::AnnouncementFindFilter { id: Some(frm.id) },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("不存在的公告")).map_err(log_error(handler_name)),
        },
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into()).map_err(log_error(handler_name));
        }
    };

    let m = model::announcement::Announcement {
        title: frm.base.title,
        content: frm.base.content,
        ..m
    };

    let aff = match m.update(&mut *tx).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into()).map_err(log_error(handler_name));
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
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/announcement/del";
    let p = get_pool(&state);
    let aff = model::announcement::Announcement::real_del(&*p, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::announcement::ListForAdmin>,
) -> Result<resp::JsonResp<model::announcement::AnnouncementPaginate>> {
    let handler_name = "admin/announcement/list";
    let p = get_pool(&state);
    let data = model::announcement::Announcement::list(
        &*p,
        &model::announcement::AnnouncementListFilter {
            pq: model::announcement::AnnouncementPaginateReq {
                page: frm.pq.page(),
                page_size: frm.pq.page_size(),
            },
            order: None,
            title: frm.title,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}
