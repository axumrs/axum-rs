use crate::model;

pub async fn add_result<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::MySql>,
    m: &'a model::Pay,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
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
