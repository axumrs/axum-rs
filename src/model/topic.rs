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

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic2AdminList {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub hit: u64,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub try_readable: bool,
    pub is_del: bool,
    pub cover: String,
    pub subject_name: String,
    pub subject_slug: String,
}

#[derive(Default)]
pub struct Topic2AdminListWith {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub subject_name: Option<String>,
    pub try_readable: Option<bool>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic2Edit {
    pub id: u64,
    pub title: String,
    pub subject_id: u32,
    pub slug: String,
    pub summary: String,
    pub author: String,
    pub src: String,
    pub cover: String,
    pub md: String,
    pub try_readable: bool,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic2WebList {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub cover: String,
    pub try_readable: bool,
    pub subject_name: String,
    pub subject_slug: String,
    pub tag_names: String,
}

#[derive(Default)]
pub struct Topic2WebListWith {
    pub title: Option<String>,
    pub subject_name: Option<String>,
    pub order_by_hit: bool,
    pub page: u32,
    pub page_size: u32,
}
