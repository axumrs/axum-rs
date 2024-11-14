use std::collections::HashMap;

use rust_decimal::Decimal;
use sqlx::PgExecutor;

use crate::{
    config,
    model::{self, currency::Currency},
    utils, Error, Result,
};

use super::Tx;

/// 验证订单金额
pub async fn valid_amount<'a>(
    e: impl PgExecutor<'a>,
    id: Option<String>,
    amount: &Decimal,
    currency: &Currency,
    cfg: &config::CurrencyConfig,
    order: Option<model::order::Order>,
) -> Result<()> {
    let m = if order.is_none() {
        if id.is_none() {
            return Err(Error::new("缺少订单id"));
        }
        match model::order::Order::find(
            e,
            &model::order::OrderFindFilter {
                id,
                user_id: None,
                status: None,
            },
        )
        .await
        {
            Ok(v) => match v {
                Some(v) => v,
                None => return Err(Error::new("不存在的订单")),
            },
            Err(e) => return Err(Error::from(e)),
        }
    } else {
        order.unwrap()
    };

    let expected_amount = match currency {
        Currency::USDT => &m.amount,
        Currency::TRX => &(&m.amount * &cfg.trx_rate),
        Currency::CNY => &(&m.amount * &cfg.cny_rate),
        Currency::PNT => &(&m.amount * &cfg.pointer_rate),
    };

    if expected_amount != amount {
        return Err(Error::new("金额不匹配"));
    }

    Ok(())
}

pub async fn update_status(
    e: impl PgExecutor<'_>,
    id: &str,
    status: &model::order::Status,
    pre_state: Option<&model::order::Status>,
) -> sqlx::Result<u64> {
    let mut q = sqlx::QueryBuilder::new("UPDATE orders SET status = ");
    q.push_bind(status);

    q.push(" WHERE id=").push_bind(id);

    if let Some(pre_state) = pre_state {
        q.push(" AND status=").push_bind(pre_state);
    }

    let aff = q.build().execute(e).await?.rows_affected();
    Ok(aff)
}

/// 更新已购服务
/// ⚠️请在外部回滚/提交事务
pub async fn update_purchased_service(
    tx: &mut Tx<'_>,
    order_id: &str,
    user_id: &str,
) -> Result<u64> {
    // 订单
    let order = match model::order::Order::find(
        &mut **tx,
        &model::order::OrderFindFilter {
            id: Some(order_id.into()),
            user_id: Some(user_id.into()),
            status: Some(model::order::Status::Finished),
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("订单不存在")),
        },
        Err(e) => return Err(Error::from(e)),
    };
    // 购买的项目
    let snap_list = order
        .to_snapshot()
        .into_iter()
        .map(|s| s.service)
        .collect::<Vec<_>>();
    for oss in snap_list {
        if oss.service.is_subject {
            // 专题
            continue;
        } else {
            // 更新用户订阅
            super::user::update_subscribe(tx, user_id, oss.service.duration, oss.num).await?;
        }
    }

    Ok(0)
}

pub async fn purchased_services(
    e: impl PgExecutor<'_>,
    user_id: &str,
    subject_ids: &[&str],
) -> Result<HashMap<String, bool>> {
    let order_list = model::order::Order::list_all(
        e,
        &model::order::OrderListAllFilter {
            limit: None,
            order: Some("id ASC".into()),
            user_id: Some(user_id.into()),
            status: Some(model::order::Status::Finished),
        },
    )
    .await?;

    let mut ps_list = HashMap::with_capacity(subject_ids.len());

    for id in subject_ids {
        ps_list.insert(id.to_string(), false);
    }

    for o in order_list {
        for oss in o.to_snapshot() {
            if oss.service.service.is_subject {
                let is_purchased =
                    utils::vec::is_in(subject_ids, &oss.service.service.target_id.as_str());
                ps_list
                    .entry(oss.service.service.target_id)
                    .and_modify(|v| *v = is_purchased);
            }
        }
    }

    Ok(ps_list)
}

pub async fn is_a_purchased_service(
    e: impl PgExecutor<'_>,
    user_id: &str,
    service_id: &str,
) -> Result<bool> {
    let psl = purchased_services(e, user_id, &[service_id]).await?;
    let r = match psl.get(service_id) {
        Some(v) => *v,
        None => false,
    };
    Ok(r)
}
