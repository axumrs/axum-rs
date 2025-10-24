use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NavPageItem {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct NavPage {
    pub id: String,
    pub prev_topic: Option<sqlx::types::Json<NavPageItem>>,
    pub next_topic: Option<sqlx::types::Json<NavPageItem>>,
}

impl NavPage {
    pub async fn find<'a>(
        e: impl sqlx::PgExecutor<'a>,
        id: &'a str,
    ) -> sqlx::Result<Option<NavPage>> {
        sqlx::query_as(
            r#"WITH topic_prev_next_cte AS (
            SELECT
                id,
                LAG(JSONB_BUILD_OBJECT('slug', slug, 'title', title)) OVER prev_next_topic AS prev_topic,
                LEAD(JSONB_BUILD_OBJECT('slug', slug, 'title', title)) OVER prev_next_topic AS next_topic
                FROM topics 
                WHERE is_del = FALSE
                WINDOW
                    prev_next_topic AS (PARTITION BY subject_id ORDER BY id ASC)
            )
            SELECT  id, prev_topic, next_topic FROM topic_prev_next_cte  WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(e)
        .await
    }
}
