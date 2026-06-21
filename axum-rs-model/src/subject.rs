use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 专题状态
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Default)]
pub enum SubjectStatus {
    #[default]
    /// 连载中
    Writing,
    /// 完结
    Finished,
}

/// 专题
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subject {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub is_del: bool,
    pub cover: String,
    pub status: SubjectStatus,
    pub price: i32,
    pub pin: i32,
}

impl Default for Subject {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: Default::default(),
            slug: Default::default(),
            summary: Default::default(),
            is_del: Default::default(),
            cover: Default::default(),
            status: Default::default(),
            price: Default::default(),
            pin: Default::default(),
        }
    }
}
