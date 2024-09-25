// use std::borrow::Cow;

use sqlx::PgExecutor;

// use crate::model;

// pub async fn batch_insert<'a>(
//     c: impl PgExecutor<'a>,
//     tcs: &[&model::topic::TopicSection],
// ) -> sqlx::Result<Vec<Cow<'a, &'a str>>> {
//     if tcs.is_empty() {
//         return Ok(vec![]);
//     }
//     let mut ids = vec![];
//     let mut q = sqlx::QueryBuilder::new("INSERT INTO ")
//     Ok(ids)
// }

/// 根据文章ID清空段落
pub async fn clean<'a>(c: impl PgExecutor<'a>, topic_id: &str) -> sqlx::Result<u64> {
    let aff = sqlx::query("DELETE FROM topic_sections WHERE topic_id=$1")
        .bind(topic_id)
        .execute(c)
        .await?
        .rows_affected();
    Ok(aff)
}
