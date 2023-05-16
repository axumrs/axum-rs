use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic {
    pub id: u64,
    pub title: String,
    pub subject_id: u32,
    pub slug: String,
    pub summary: String,
    pub author: String,
    pub src: String,
    pub hit: u64,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub try_readable: bool,
    pub is_del: bool,
    pub cover: String,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct TopicContent {
    pub topic_id: u64,
    pub md: String,
    pub html: String,
}
