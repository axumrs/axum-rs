use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = read_histories, pk = id)]
pub struct ReadHistorie {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    #[db(find_opt)]
    #[db(skip_update)]
    #[db(list_opt)]
    pub user_id: String,

    #[db(skip_update)]
    pub subject_slug: String,

    #[db(skip_update)]
    pub slug: String,

    #[db(skip_update)]
    pub subject_name: String,

    #[db(skip_update)]
    pub topic_title: String,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,
}
