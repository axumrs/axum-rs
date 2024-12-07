use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = announcements, pk = id)]
pub struct Announcement {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    #[db(list_opt)]
    #[db(list_opt_like)]
    pub title: String,

    pub content: String,
    pub hit: i64,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct AnnouncementLite {
    pub id: String,
    pub title: String,
    pub dateline: DateTime<Local>,
}
