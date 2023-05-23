use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserLoginLogFull {
    pub id: u64,
    pub user_id: u64,
    pub ip: String,
    pub browser: String,
    pub os: String,
    pub device: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
    pub user_agent: String,
}
