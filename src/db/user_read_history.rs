use crate::{model, Error, Result};

use super::Paginate;

pub async fn add(conn: &sqlx::MySqlPool, m: &model::UserReadHistory) -> Result<u64> {
    let id = sqlx::query("INSERT INTO user_read_history (user_id, subject_slug, slug, dateline, is_del) VALUES(?,?,?,?,?)")
    .bind(&m.user_id)
    .bind(&m.subject_slug)
    .bind(&m.slug)
    .bind(&m.dateline)
    .bind(&m.is_del)
    .execute(conn).await.map_err(Error::from)?
    .last_insert_id();
    Ok(id)
}

pub async fn list(
    conn: &sqlx::MySqlPool,
    with: &model::UserReadHistoryListWith,
) -> Result<Paginate<model::UserReadHistoryListView>> {
    let mut q = sqlx::QueryBuilder::new(
        "SELECT id, dateline, is_del, topic_id, title, slug, try_readable, cover, summary, hit, subject_name, subject_slug, tag_names, user_id, email, nickname FROM v_user_read_history WHERE 1=1",
    );
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_user_read_history WHERE 1=1");

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
