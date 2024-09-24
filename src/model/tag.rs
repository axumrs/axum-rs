use axum_rs_derive::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = tags, pk = id)]
pub struct Tag {
    #[db(skip_update)]
    #[db(find_opt)]
    pub id: String,

    #[db(exists)]
    #[db(find_opt)]
    #[db(list_opt)]
    #[db(list_opt_like)]
    pub name: String,

    pub is_del: bool,
}
