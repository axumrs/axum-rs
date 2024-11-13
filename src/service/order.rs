use rust_decimal::Decimal;
use sqlx::PgExecutor;

use crate::{
    config,
    model::{self, currency::Currency},
    Error, Result,
};

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
