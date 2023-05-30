use crate::{model, Error, Result};

pub async fn exists(conn: &sqlx::MySqlPool, order_id: u64, user_id: u32) -> Result<bool> {
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM pay_apply WHERE order_id=? AND user_id=?")
            .bind(order_id)
            .bind(user_id)
            .fetch_one(conn)
            .await
            .map_err(Error::from)?;
    Ok(count.0 > 0)
}

pub async fn add(conn: &sqlx::MySqlPool, m: &model::PayApply) -> Result<u64> {
    if exists(conn, m.order_id, m.user_id).await? {
        return Err(Error::already_exists("该订单已提交过支付证明"));
    }
    let id = sqlx::query("INSERT INTO pay_apply (order_id, user_id, price, currency, types, tx_id, status, dateline, is_del, img, process_dateline, reason) VALUES(?,?,?,?,?,?,?,?,?,?,?,?)")
    .bind(&m.order_id)
    .bind(&m.user_id)
    .bind(&m.price)
    .bind(&m.currency)
    .bind(&m.types)
    .bind(&m.tx_id)
    .bind(&m.status)
    .bind(&m.dateline)
    .bind(&m.is_del)
    .bind(&m.img)
    .bind(&m.process_dateline)
    .bind(&m.reason)
    .execute(conn)
    .await
    .map_err(Error::from)?
    .last_insert_id();

    Ok(id)
}

pub async fn find(
    conn: &sqlx::MySqlPool,
    by: &model::PayApplyFindBy,
) -> Result<Option<model::PayApply>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id,order_id, user_id, price, currency, types, tx_id, status, dateline, is_del, img, process_dateline, reason FROM pay_apply WHERE 1=1");

    match by {
        &model::PayApplyFindBy::ID(id) => q.push(" AND id=").push_bind(id),
        &model::PayApplyFindBy::Owner { order_id, user_id } => q
            .push(" AND order_id=")
            .push_bind(order_id)
            .push(" AND user_id=")
            .push_bind(user_id),
    };

    q.push(" LIMIT 1");

    let r = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;

    Ok(r)
}

/// 拒绝
pub async fn reject(conn: &sqlx::MySqlPool, id: u64, reason: &str) -> Result<u64> {
    let aff = sqlx::query("UPDATE pay_apply SET status=?,process_dateline=?, reason=? WHERE id=?")
        .bind(&model::PayApplyStatus::Reject)
        .bind(chrono::Local::now())
        .bind(reason)
        .bind(id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

/// 接受
pub async fn accept(
    conn: &sqlx::MySqlPool,
    pay_apply: &model::PayApply,
    pay: &model::Pay,
) -> Result<u64> {
    super::order::update_with_pay(
        conn,
        pay_apply.order_id,
        pay_apply.user_id,
        pay,
        Some(pay_apply),
    )
    .await
}
