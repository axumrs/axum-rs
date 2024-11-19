use axum::{
    extract::{Path, Query, State},
    Json,
};
use rust_decimal::Decimal;
use sqlx::QueryBuilder;
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
        &model::service::ServiceFindFilter {
            id: Some(frm.id),
            is_subject: None,
            target_id: None,
        },
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
    Query(frm): Query<form::service::ListForAdmin>,
) -> Result<resp::JsonResp<model::service::ServicePaginate>> {
    let handler_name = "admin/service/list";
    let p = get_pool(&state);

    let data = model::service::Service::list(
        &*p,
        &model::service::ServiceListFilter {
            pq: model::service::ServicePaginateReq {
                page: frm.pq.page(),
                page_size: frm.pq.page_size(),
            },
            is_off: frm.is_off(),
            is_subject: frm.is_subject(),
            order: Some("pin DESC, id DESC".into()),
            name: frm.name,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn on_off(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/service/on_off";
    let p = get_pool(&state);

    let aff = sqlx::query("UPDATE services SET is_off = (NOT is_off) WHERE id = $1")
        .bind(&id)
        .execute(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?
        .rows_affected();

    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn del(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/service/del";
    let p = get_pool(&state);
    let aff = model::service::Service::real_del(&*p, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn import(State(state): State<ArcAppState>) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/service/import";
    let p = get_pool(&state);

    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let subject_list = match model::subject::Subject::list_all(
        &mut *tx,
        &model::subject::SubjectListAllFilter {
            is_del: Some(false),
            limit: None,
            name: None,
            slug: None,
            status: None,
            order: None,
        },
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

    let subject_list = subject_list
        .into_iter()
        .filter(|s| s.price > Decimal::ZERO)
        .collect::<Vec<_>>();

    let mut service_list = vec![];

    for s in subject_list {
        let s_is_exists =
            match model::service::Service::target_id_is_exists(&mut *tx, &s.id, None).await {
                Ok(v) => v,
                Err(e) => {
                    tx.rollback()
                        .await
                        .map_err(Error::from)
                        .map_err(log_error(handler_name))?;
                    return Err(e.into());
                }
            };
        if s_is_exists {
            continue;
        }
        let name_is_exists =
            match model::service::Service::name_is_exists(&mut *tx, &s.name, None).await {
                Ok(v) => v,
                Err(e) => {
                    tx.rollback()
                        .await
                        .map_err(Error::from)
                        .map_err(log_error(handler_name))?;
                    return Err(e.into());
                }
            };
        let name = if name_is_exists {
            format!("{}#{}", s.id, s.name)
        } else {
            s.name
        };

        let name = utils::str::fixlen(&name, 100).to_string();

        service_list.push(model::service::Service {
            id: utils::id::new(),
            name,
            is_subject: true,
            target_id: s.id,
            price: s.price,
            cover: s.cover,
            is_off: false,
            desc: s.summary,
            ..Default::default()
        });
    }

    let mut aff = 0;
    if !service_list.is_empty() {
        let mut q = sqlx::QueryBuilder::new(
            r#"INSERT INTO services (id, "name", is_subject, target_id, duration, price, cover, allow_pointer, normal_discount, sub_discount, yearly_sub_discount, is_off, "desc", pin) "#,
        );
        q.push_values(&service_list, |mut b, s| {
            b.push_bind(&s.id)
                .push_bind(&s.name)
                .push_bind(&s.is_subject)
                .push_bind(&s.target_id)
                .push_bind(&s.duration)
                .push_bind(&s.price)
                .push_bind(&s.cover)
                .push_bind(&s.allow_pointer)
                .push_bind(&s.normal_discount)
                .push_bind(&s.sub_discount)
                .push_bind(&s.yearly_sub_discount)
                .push_bind(&s.is_off)
                .push_bind(&s.desc)
                .push_bind(&s.pin);
        });
        aff = match q.build().execute(&mut *tx).await {
            Ok(v) => v.rows_affected(),
            Err(e) => {
                tx.rollback()
                    .await
                    .map_err(Error::from)
                    .map_err(log_error(handler_name))?;
                return Err(e.into());
            }
        };
    }

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn sync(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/service/sync";
    let p = get_pool(&state);

    let s = match model::service::Service::find(
        &*p,
        &model::service::ServiceFindFilter {
            id: Some(id),
            is_subject: None,
            target_id: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("不存在的服务")),
    };

    let sub = match model::subject::Subject::find(
        &*p,
        &model::subject::SubjectFindFilter {
            by: model::subject::SubjectFindBy::Id(s.target_id.clone()),
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

    let s = model::service::Service {
        name: sub.name,
        price: sub.price,
        cover: sub.cover,
        desc: sub.summary,
        ..s
    };

    let aff = s
        .update(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn search(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::service::SearchForAdmin>,
) -> Result<resp::JsonResp<Vec<model::service::Service>>> {
    let handler_name = "admin/service/search";
    let p = get_pool(&state);

    let sql = format!(
        "SELECT {} FROM {} WHERE 1=1",
        &model::service::Service::fields(),
        &model::service::Service::table()
    );

    let mut q = QueryBuilder::new(&sql);

    if let Some(ids) = frm.ids() {
        q.push(" AND id IN");
        q.push_tuples(ids, |mut b, id| {
            b.push_bind(id);
        });
    } else {
        q.push(" AND name ILIKE ").push_bind(&frm.q);
    }

    q.push(" ORDER BY id DESC LIMIT 30");

    let rows = q
        .build_query_as()
        .fetch_all(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(rows))
}

pub async fn list_all(
    State(state): State<ArcAppState>,
) -> Result<resp::JsonResp<Vec<model::service::Service>>> {
    let handler_name = "admin/service/list_all";
    let p = get_pool(&state);

    let rows = model::service::Service::list_all(
        &*p,
        &model::service::ServiceListAllFilter {
            limit: None,
            order: None,
            name: None,
            is_subject: None,
            is_off: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(rows))
}
