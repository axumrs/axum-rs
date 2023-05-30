use crate::{model, Error, Result};

use super::Paginate;

pub async fn find(
    conn: &sqlx::MySqlPool,
    id: u64,
    status: Option<model::UserPurchasedServiceStatus>,
    user_id: Option<u32>,
) -> Result<Option<model::UserPurchasedServiceFull>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, order_id, user_id, service_id, service_type, server_num, status, dateline, email, nickname, subject_id, subject_slug, subject_name, subject_summary, subject_cover, subject_status, subject_price, subject_is_del, order_num FROM v_user_purchased_service WHERE id=");
    q.push_bind(id);

    if let Some(user_id) = user_id {
        q.push(" AND user_id=").push_bind(user_id);
    }

    if let Some(status) = status {
        let sql = " AND status=";
        q.push(sql).push_bind(status);
    }

    q.push(" LIMIT 1");

    let r = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;

    Ok(r)
}

pub async fn list(
    conn: &sqlx::MySqlPool,
    with: &model::UserPurchasedServiceFullListWith,
) -> Result<Paginate<model::UserPurchasedServiceFull>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, order_id, user_id, service_id, service_type, server_num, status, dateline, email, nickname, subject_id, subject_slug, subject_name, subject_summary, subject_cover, subject_status, subject_price, subject_is_del, order_num FROM v_user_purchased_service WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_user_purchased_service WHERE 1=1");

    if let Some(user_id) = &with.user_id {
        let sql = " AND user_id=";

        q.push(sql).push_bind(user_id);
        qc.push(sql).push_bind(user_id);
    }

    q.push(" ORDER BY id DESC")
        .push(" LIMIT ")
        .push_bind(with.pw.page_size)
        .push(" OFFSET ")
        .push_bind(with.pw.offset());

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

    Ok(Paginate::with(&count, &with.pw, data))
}
