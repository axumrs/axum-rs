use sqlx::PgExecutor;

/// 根据文章ID清空标签
pub async fn clean<'a>(c: impl PgExecutor<'a>, topic_id: &str) -> sqlx::Result<u64> {
    let aff = sqlx::query("DELETE FROM topic_tags WHERE topic_id=$1")
        .bind(topic_id)
        .execute(c)
        .await?
        .rows_affected();
    Ok(aff)
}

/// 根据标签ID清空文章
pub async fn clean_by_tag<'a>(c: impl PgExecutor<'a>, tag_id: &str) -> sqlx::Result<u64> {
    let aff = sqlx::query("DELETE FROM topic_tags WHERE tag_id=$1")
        .bind(tag_id)
        .execute(c)
        .await?
        .rows_affected();
    Ok(aff)
}
