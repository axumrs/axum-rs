use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = sessions, pk = id)]
pub struct Session {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    #[db(find_opt)]
    #[db(skip_update)]
    #[db(list_opt)]
    pub user_id: String,

    #[db(skip_update)]
    #[db(list_opt)]
    pub token: String,

    #[db(skip_update)]
    #[db(list_opt)]
    pub is_admin: bool,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,

    #[db(skip_update)]
    pub ip: String,

    #[db(skip_update)]
    pub ua: String,

    #[db(skip_update)]
    pub loc: String,

    #[db(skip_update)]
    #[db(list_opt)]
    #[db(list_opt_between)]
    #[db(find_opt)]
    #[db(find_opt_between)]
    pub expire_time: DateTime<Local>,
}
