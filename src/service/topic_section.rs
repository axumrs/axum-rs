// use std::borrow::Cow;

// use sqlx::PgExecutor;

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
