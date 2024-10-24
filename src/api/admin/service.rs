use axum::{
    extract::{Query, State},
    Json,
};
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, model, resp, utils, ArcAppState, Error, Result,
};

pub async fn add(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::service::Add>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "admin/service/add";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let name_exists = match model::service::Service::name_is_exists(&mut *tx, &frm.name, None).await
    {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    if name_exists {
        return Err(Error::new("服务已存在"));
    }

    if frm.is_subject {
        let subject_exists = match model::service::Service::target_id_is_exists(
            &mut *tx,
            &frm.target_id,
            None,
        )
        .await
        {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        };

        if subject_exists {
            return Err(Error::new("服务已存在"));
        }
    }

    let m = model::service::Service {
        id: utils::id::new(),
        name: frm.name,
        is_subject: frm.is_subject,
        target_id: frm.target_id,
        duration: frm.duration,
        price: frm.price,
        cover: frm.cover,
        allow_pointer: frm.allow_pointer,
        normal_discount: frm.normal_discount,
        sub_discount: frm.sub_discount,
        yearly_sub_discount: frm.yearly_sub_discount,
        is_off: frm.is_off,
        desc: frm.desc,
        pin: frm.pin,
    };

    let id = match m.insert(&mut *tx).await {
        Ok(id) => id,
        Err(e) => return Err(e.into()),
    };

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::IDResp { id }))
}

pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::service::Edit>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/service/edit";
    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let name_exists = match model::service::Service::name_is_exists(
        &mut *tx,
        &frm.base.name,
        Some(frm.id.clone()),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into());
        }
    };

    if name_exists {
        return Err(Error::new("服务已存在"));
    }

    if frm.base.is_subject {
        let subject_exists = match model::service::Service::target_id_is_exists(
            &mut *tx,
            &frm.base.target_id,
            Some(frm.id.clone()),
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                tx.rollback()
                    .await
                    .map_err(Error::from)
                    .map_err(log_error(handler_name))?;
                return Err(e.into());
            }
        };

        if subject_exists {
            return Err(Error::new("服务已存在"));
        }
    }

    let m = match model::service::Service::find(
        &mut *tx,
        &model::service::ServiceFindFilter { id: Some(frm.id) },
    )
    .await
    {
        Ok(m) => {
            if let Some(v) = m {
                v
            } else {
                return Err(Error::new("服务不存在"));
            }
        }
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into());
        }
    };

    let m = model::service::Service {
        name: frm.base.name,
        is_subject: frm.base.is_subject,
        target_id: frm.base.target_id,
        duration: frm.base.duration,
        price: frm.base.price,
        cover: frm.base.cover,
        allow_pointer: frm.base.allow_pointer,
        normal_discount: frm.base.normal_discount,
        sub_discount: frm.base.sub_discount,
        yearly_sub_discount: frm.base.yearly_sub_discount,
        is_off: frm.base.is_off,
        desc: frm.base.desc,
        pin: frm.base.pin,
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

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::PageQuery>,
) -> Result<resp::JsonResp<model::service::ServicePaginate>> {
    let handler_name = "admin/service/list";
    let p = get_pool(&state);

    let data = model::service::Service::list(
        &*p,
        &model::service::ServiceListFilter {
            pq: model::service::ServicePaginateReq {
                page: frm.page(),
                page_size: frm.page_size(),
            },
            order: None,
            name: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}
