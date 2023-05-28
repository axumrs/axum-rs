use serde::{Deserialize, Serialize};

use super::PaginateWith;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserReadHistory {
    pub id: u64,
    pub user_id: u32,
    pub subject_slug: String,
    pub slug: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
}

#[derive(Default)]
pub struct UserReadHistoryListWith {
    pub user_id: Option<u32>,
    pub pw: PaginateWith,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserReadHistoryListView {
    pub id: u64,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
    pub topic_id: u64,
    pub title: String,
    pub slug: String,
    pub try_readable: bool,
    pub cover: String,
    pub summary: String,
    pub hit: u64,
    pub subject_name: String,
    pub subject_slug: String,
    pub tag_names: String,
    pub user_id: u32,
    pub email: String,
    pub nickname: String,
}
