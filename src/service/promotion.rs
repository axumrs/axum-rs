use sqlx::PgExecutor;

use crate::{model, Result};

type Model = model::promotion::Promotion;

pub async fn random_take(c: impl PgExecutor<'_>) -> Result<Option<Model>> {
    let sql = format!(
        "SELECT {} FROM {} ORDER BY RANDOM() LIMIT 1",
        &Model::fields(),
        &Model::table()
    );
    let r = sqlx::query_as(&sql).fetch_optional(c).await?;
    Ok(r)
}
