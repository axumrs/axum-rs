use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = check_in_logs, pk = id)]
pub struct CheckInLog {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    #[db(find_opt)]
    #[db(skip_update)]
    #[db(list_opt)]
    pub user_id: String,

    #[db(skip_update)]
    pub points: i16,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,
}
