use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = topics, pk = id)]
pub struct Topic {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    pub title: String,

    #[db(find_opt)]
    pub subject_id: String,

    #[db(find_opt)]
    pub slug: String,

    pub summary: String,
    pub author: String,
    pub src: String,
    pub hit: i64,
    pub dateline: DateTime<Local>,
    pub try_readable: bool,
    pub is_del: bool,
    pub cover: String,
    pub md: String,

    #[db(find_opt)]
    pub ver: i32,
    pub pin: i32,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = topic_sections, pk = id)]
pub struct TopicSection {
    #[db(find)]
    #[db(skip_update)]
    pub id: String,

    #[db(list_opt)]
    pub topic_id: String,

    pub sort: i32,

    #[db(find_opt)]
    pub ver: i32,

    pub pre_id: String,
    pub content: String,
}
