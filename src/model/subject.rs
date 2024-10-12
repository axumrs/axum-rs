use axum_rs_derive::Db;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "subject_status")]
pub enum Status {
    #[default]
    Writing,
    Finished,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = subjects, pk = id, del_field = is_del)]
pub struct Subject {
    #[db(find)]
    #[db(skip_update)]
    pub id: String,

    #[db(list_opt)]
    #[db(list_opt_like)]
    pub name: String,

    #[db(find)]
    #[db(exists)]
    #[db(list_opt)]
    #[db(list_opt_like)]
    pub slug: String,

    pub summary: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub is_del: bool,
    pub cover: String,

    #[db(list_opt)]
    pub status: Status,

    pub price: Decimal,
    pub pin: i32,
}
