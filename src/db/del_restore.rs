use crate::{Error, Result};

pub enum PrimaryKey {
    Int(u32),
    BigInt(u64),
}

pub async fn invoke<'a, C>(conn: C, table: &'a str, id: PrimaryKey, is_del: bool) -> Result<u64>
where
    C: sqlx::MySqlExecutor<'a>,
{
    let mut q = sqlx::QueryBuilder::new("UPDATE ");
    q.push(table);
    q.push(" SET is_del = ");
    q.push_bind(is_del);
    q.push(" WHERE id = ");
    match id {
        PrimaryKey::Int(id) => q.push_bind(id),
        PrimaryKey::BigInt(id) => q.push_bind(id),
    };

    let aff = q
        .build()
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(aff)
}
