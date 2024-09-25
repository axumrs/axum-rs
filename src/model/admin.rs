use axum_rs_derive::Db;
use serde::{Deserialize, Serialize};

use crate::interfaces;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = admins, pk = id)]
pub struct Admin {
    #[db(skip_update)]
    #[db(find)]
    pub id: String,

    #[db(find)]
    #[db(skip_update)]
    #[db(exists)]
    pub username: String,

    #[serde(skip_serializing)]
    pub password: String,
}

impl interfaces::AsAuth for Admin {}
