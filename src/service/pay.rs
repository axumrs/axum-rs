use chrono::Local;
use sqlx::{PgExecutor, PgPool};

use crate::{config, model, tron, utils, Error, Result};

pub async fn create(
    p: &PgPool,
    m: model::pay::Pay,
    cfg: &config::CurrencyConfig,
    re_pay: bool,
) -> Result<model::pay::Pay> {
    let mut tx = p.begin().await?;
    if re_pay {
        if let Err(e) = sqlx::query("DELETE FROM pays WHERE order_id = $1")
            .bind(&m.order_id)
            .execute(&mut *tx)
            .await
        {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(e));
        }
    }
    let order_is_exists =
        match model::pay::Pay::order_id_is_exists(&mut *tx, &m.order_id, None).await {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(Error::from(e));
            }
        };
    if order_is_exists {
        return Err(Error::new("订单已存在支付记录"));
    }

    if let Err(e) = super::order::valid_amount(
        &mut *tx,
        Some(m.order_id.clone()),
        &m.amount,
        &m.currency,
        cfg,
        None,
    )
    .await
    {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e);
    }

    let id = utils::id::new();
    let m = model::pay::Pay {
        id,
        dateline: Local::now(),
        ..m
    };
    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::from(e));
    }

    tx.commit().await?;
    Ok(m)
}

pub async fn find(
    e: impl PgExecutor<'_>,
    f: &model::pay::PayFindFilter,
) -> Result<Option<model::pay::Pay>> {
    model::pay::Pay::find(e, f).await.map_err(Error::from)
}

pub async fn complete(
    // p: &PgPool,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: String,                 // 支付ID
    order_id: String,           // 订单ID
    cfg: &config::Config,       // 配置
    user_id: Option<String>,    // 用户ID
    skip_check_confirmed: bool, // 是否跳过区块链确认
) -> Result<u64> {
    // 获取支付记录
    let pay = match find(
        &mut **tx,
        &model::pay::PayFindFilter {
            id: Some(id),
            order_id: Some(order_id),
            user_id: user_id.clone(),
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("支付记录不存在")),
        },
        Err(e) => {
            return Err(Error::from(e));
        }
    };
    // 获取订单
    let order = match model::order::Order::find(
        &mut **tx,
        &model::order::OrderFindFilter {
            id: Some(pay.order_id),
            user_id,
            status: Some(model::order::Status::Pending),
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("订单不存在")),
        },
        Err(e) => {
            return Err(Error::from(e));
        }
    };
    let order_id = order.id.clone();
    let order_user_id = order.user_id.clone();

    let mut amount = order.amount.clone();
    // 区块链是否确认
    let is_online = matches!(pay.method, model::pay::Method::Online);
    if is_online && !skip_check_confirmed {
        match &pay.currency {
            &model::currency::Currency::USDT => {
                let tron_tx = match tron::usdt_tran(&cfg.tron, &pay.tx_id).await {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };
                if !tron_tx.is_valid(&cfg.tron, &order.amount)? {
                    return Err(Error::new("区块链交易无效"));
                }
                amount = tron_tx.amount();
            }
            &model::currency::Currency::TRX => {
                let tron_tx = match tron::trx_tran(&cfg.tron, &pay.tx_id).await {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };
                if !tron_tx.is_valid(&cfg.tron, &(&order.amount * &cfg.currency.trx_rate))? {
                    return Err(Error::new("区块链交易无效"));
                }
                amount = tron_tx.amount();
            }
            _ => {
                unreachable!()
            }
        }
    }

    // 校验金额 区块链->订单
    if let Err(e) = super::order::valid_amount(
        &mut **tx,
        None,
        &amount,
        &pay.currency,
        &cfg.currency,
        Some(order),
    )
    .await
    {
        return Err(e);
    }

    // 订单状态
    if let Err(e) = super::order::update_status(
        &mut **tx,
        &order_id,
        &model::order::Status::Finished,
        Some(&model::order::Status::Pending),
    )
    .await
    {
        return Err(Error::from(e));
    }
    // 支付状态
    if let Err(e) = update_status(
        &mut **tx,
        &pay.id,
        &model::pay::Status::Success,
        Some(&model::pay::Status::Pending),
    )
    .await
    {
        return Err(Error::from(e));
    }

    // 已购服务
    if let Err(e) = super::order::update_purchased_service(tx, &order_id, &order_user_id).await {
        return Err(Error::from(e));
    }

    Ok(0)
}

pub async fn update_status(
    e: impl PgExecutor<'_>,
    id: &str,
    status: &model::pay::Status,
    pre_state: Option<&model::pay::Status>,
) -> sqlx::Result<u64> {
    let mut q = sqlx::QueryBuilder::new("UPDATE pays SET status = ");
    q.push_bind(status);

    q.push(" WHERE id=").push_bind(id);

    if let Some(pre_state) = pre_state {
        q.push(" AND status=").push_bind(pre_state);
    }

    let aff = q.build().execute(e).await?.rows_affected();
    Ok(aff)
}
