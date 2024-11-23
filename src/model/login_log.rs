use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = login_logs, pk = id)]
pub struct LoginLog {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    #[db(find_opt)]
    #[db(skip_update)]
    #[db(list_opt)]
    pub user_id: String,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,

    #[db(skip_update)]
    pub ip: String,

    #[db(skip_update)]
    pub user_agent: String,
}
