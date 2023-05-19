use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Admin {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub is_del: bool,
}

pub enum AdminFindBy<'a> {
    ID(u32),
    Username(&'a str),
}

#[derive(Default)]
pub struct Admin2Edit {
    pub id: u32,
    pub username: String,
    pub password: Option<String>,
}
