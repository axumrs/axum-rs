use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum SubjectStatus {
    #[default]
    Writing = 0,
    Finished = 1,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct Subject {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub is_del: bool,
    pub cover: String,
    pub status: SubjectStatus,
    pub price: u32,
}

pub enum SubjectFindBy<'a> {
    ID(u32),
    Slug(&'a str),
}

#[derive(Default)]
pub struct SubjectListWith {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub status: Option<SubjectStatus>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}
