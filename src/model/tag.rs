use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub is_del: bool,
}

pub enum TagFindBy<'a> {
    ID(u32),
    /// 模糊
    Name(&'a str),
    /// 严格
    ExactName(&'a str),
}

#[derive(Default)]
pub struct TagListWith {
    pub name: Option<String>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Tag2TopicEdit {
    pub name: String,
}
