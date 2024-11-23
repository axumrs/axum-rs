use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = protected_contents, pk = id)]
pub struct ProtectedContent {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    #[db(find_opt)]
    #[db(skip_update)]
    #[db(list_opt)]
    pub section_id: String,

    #[db(skip_update)]
    pub content: String,

    #[db(skip_update)]
    #[db(list_opt)]
    #[db(list_opt_between)]
    #[db(find_opt)]
    #[db(find_opt_between)]
    pub expire_time: DateTime<Local>,
}
