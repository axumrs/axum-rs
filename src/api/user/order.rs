use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Local;
use rust_decimal::Decimal;
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, mid, model, resp, utils, ArcAppState, Error, Result,
};

pub async fn create(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Json(frm): Json<form::order::Create>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "user/order/create";

    let user = user_auth.user().map_err(log_error(handler_name))?;

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if frm.services.is_empty() {
        return Err(Error::new("订购的服务不能为空"));
    }

    let p = get_pool(&state);

    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 服务列表
    let service_ids = frm
        .services
        .iter()
        .map(|s| s.id.clone())
        .collect::<Vec<_>>();
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT id, "name", is_subject, target_id, duration, price, cover, allow_pointer, normal_discount, sub_discount, yearly_sub_discount, is_off, "desc", pin FROM services WHERE id IN "#,
    );
    q.push_tuples(&service_ids, |mut b, id| {
        b.push_bind(id);
    });
    let service_list: Vec<model::service::Service> =
        match q.build_query_as().fetch_all(&mut *tx).await {
            Ok(v) => v,
            Err(e) => {
                tx.rollback()
                    .await
                    .map_err(Error::from)
                    .map_err(log_error(handler_name))?;
                return Err(e.into()).map_err(log_error(handler_name));
            }
        };

    // 快照
    let mut snapshot_list = Vec::with_capacity(service_list.len());
    for (idx, s) in service_list.into_iter().enumerate() {
        let num = match frm.services.get(idx) {
            Some(v) => v.num,
            None => return Err(Error::new("服务不存在")),
        };
        let amount = s.price * Decimal::from_i128_with_scale(num as i128, 0);
        let discount = 1; // TODO: 折扣<后续版本>
        let actual_price = s.price * Decimal::from_i128_with_scale(discount as i128, 0);
        let actual_amount = actual_price * Decimal::from_i128_with_scale(num as i128, 0);
        let snap_service = model::order::OrderSnapshotService {
            service: model::service::Service { ..s },
            actual_price,
            amount,
            actual_amount,
            discount,
            num,
        };
        let snap = model::order::OrderSnapshot {
            service: snap_service,
            user: user.clone(),
        };
        snapshot_list.push(snap);
    }

    // 计算总价
    let mut amount = Decimal::ZERO;
    let mut actual_amount = Decimal::ZERO;
    for s in snapshot_list.iter() {
        amount += s.service.amount;
        actual_amount += s.service.actual_amount;
    }

    // 检测提交的金额和实际金额是否一致
    if !(frm.amount == amount && frm.actual_amount == actual_amount) {
        return Err(Error::new("订单金额不符")).map_err(log_error(handler_name));
    }

    let snapshot = model::order::Order::snapshot_to_str(&snapshot_list);
    let id = utils::id::new();

    let m = model::order::Order {
        id,
        user_id: user.id.clone(),
        dateline: Local::now(),
        status: model::order::Status::Pending,
        amount,
        actual_amount,
        snapshot,
        allow_pointer: false,
        ..Default::default()
    };

    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into()).map_err(log_error(handler_name));
    }

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::IDResp { id: m.id }))
}

pub async fn list(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Query(frm): Query<form::PageQueryStr>,
) -> Result<resp::JsonResp<model::order::OrderPaginate>> {
    let handler_name = "user/order/list";

    let user = user_auth.user().map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let data = model::order::Order::list(
        &*p,
        &model::order::OrderListFilter {
            pq: model::order::OrderPaginateReq {
                page: frm.page(),
                page_size: frm.page_size(),
            },
            order: None,
            user_id: Some(user.id.clone()),
            status: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn detail(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Path(id): Path<String>,
) -> Result<resp::JsonResp<model::order::Order>> {
    let handler_name = "user/order/detail";

    let user = user_auth.user().map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let data = match model::order::Order::find(
        &*p,
        &model::order::OrderFindFilter {
            id: Some(id),
            user_id: Some(user.id.clone()),
            status: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("不存在的订单")),
    };

    Ok(resp::ok(data))
}

pub async fn cancel(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "user/order/cancel";
    let user = user_auth.user().map_err(log_error(handler_name))?;

    let p = get_pool(&state);
    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let o = match model::order::Order::find(
        &mut *tx,
        &model::order::OrderFindFilter {
            id: Some(id),
            user_id: Some(user.id.clone()),
            status: Some(model::order::Status::Pending),
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("不存在的订单")),
        },
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(Error::from(e)).map_err(log_error(handler_name))?;
        }
    };

    let o = model::order::Order {
        status: model::order::Status::Cancelled,
        ..o
    };

    let aff = match o.update(&mut *tx).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(Error::from(e)).map_err(log_error(handler_name))?;
        }
    };

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}
