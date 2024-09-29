use axum_rs_derive::Db;
use chrono::{DateTime, Local};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone)]
#[sqlx(type_name = "activation_kind")]
pub enum Kind {
    #[default]
    Register,
    ResetPassword,
    ChangeEmail,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = activation_codes, pk = id)]
pub struct ActivationCode {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,

    #[db(find_opt)]
    #[db(skip_update)]
    pub email: String,

    #[db(skip_update)]
    #[db(find_opt)]
    pub code: String,

    #[db(find_opt)]
    pub kind: Kind,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,
}
