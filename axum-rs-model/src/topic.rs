use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicBase {
    pub id: Uuid,
    pub title: String,
    pub subject_id: Uuid,
    pub slug: String,
    pub summary: String,
    pub hit: i64,
    pub dateline: DateTime<Utc>,
    pub try_readable: bool,
    pub is_del: bool,
    pub cover: String,
    pub tags: Vec<String>,
    pub pin: i32,
}
impl Default for TopicBase {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            title: Default::default(),
            subject_id: Default::default(),
            slug: Default::default(),
            summary: Default::default(),
            hit: Default::default(),
            dateline: Utc::now(),
            try_readable: Default::default(),
            is_del: Default::default(),
            cover: Default::default(),
            tags: Default::default(),
            pin: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct Topic {
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub base: TopicBase,
    pub author: String,
    pub src: String,
    pub md: String,
    pub sections: Vec<String>,
}
