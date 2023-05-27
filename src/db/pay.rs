use crate::{model, Error, Result};

pub async fn add_result<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::MySql>,
    m: &'a model::Pay,
) -> std::result::Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO pay (order_id, user_id, price, currency, types, tx_id, status, dateline, is_del) VALUES(?,?,?,?,?,?,?,?,?)")
        .bind(&m.order_id)
        .bind(&m.user_id)
        .bind(m.price*100)
        .bind(&m.currency)
        .bind(&m.types)
        .bind(&m.tx_id)
        .bind(&m.status)
        .bind(&m.dateline)
        .bind(&m.is_del)
        .execute(tx)
        .await
}

pub async fn find(conn: &sqlx::MySqlPool, id: u64) -> Result<Option<model::Pay>> {
    let  p = sqlx::query_as("SELECT id,order_id, user_id, price, currency, types, tx_id, status, dateline, is_del FROM pay WHERE id=?").bind(id).fetch_optional(conn).await.map_err(Error::from)?;

    Ok(p)
}
