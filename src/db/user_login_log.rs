use crate::{model, Error, Result};

use super::Paginate;

pub async fn list(
    conn: &sqlx::MySqlPool,
    with: &model::PaginateWith,
    user_id: u32,
) -> Result<Paginate<model::UserLoginLogFull>> {
    let mut q = sqlx::QueryBuilder::new ("SELECT id, user_id, ip, browser, os, device, dateline, is_del, user_agent FROM v_user_login_log_full");
    q.push(" WHERE user_id=").push_bind(user_id);

    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_user_login_log_full");
    qc.push(" WHERE user_id=").push_bind(user_id);

    q.push(" ORDER BY id DESC")
        .push(" LIMIT ")
        .push_bind(with.page_size)
        .push(" OFFSET ")
        .push_bind(with.page * with.page_size);

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    let data = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(Paginate::new(
        count.0 as u32,
        with.page,
        with.page_size,
        data,
    ))
}
