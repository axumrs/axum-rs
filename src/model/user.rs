use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_status")]
pub enum Status {
    #[default]
    Pending,
    Actived,
    Freezed,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_kind")]
pub enum Kind {
    #[default]
    Normal,
    Subscriber,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = users, pk = id)]
pub struct User {
    #[db(find)]
    #[db(skip_update)]
    pub id: String,

    #[db(find)]
    #[db(skip_update)]
    pub email: String,

    pub nickname: String,

    #[serde(skip_serializing)]
    pub password: String,
    pub status: Status,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,

    pub kind: Kind,
    pub sub_exp: DateTime<Local>,
    pub points: Decimal,
    pub allow_device_num: i16,
    pub session_exp: i16,
}
