use serde::Deserialize;
use validator::Validate;

use crate::model;

#[derive(Deserialize, Validate)]
pub struct Create {
    #[validate(length(max = 100))]
    pub name: String,
    #[validate(length(max = 100))]
    pub slug: String,
    #[validate(length(max = 255))]
    pub summary: String,
    #[validate(length(max = 100))]
    pub cover: String,
    #[validate(range(min = 0))]
    pub price: u32,
    pub status: Option<model::SubjectStatus>,
}

#[derive(Deserialize)]
pub struct List {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub status: Option<model::SubjectStatus>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Deserialize, Validate)]
pub struct Update {
    #[validate(length(max = 100))]
    pub name: String,
    #[validate(length(max = 100))]
    pub slug: String,
    #[validate(length(max = 255))]
    pub summary: String,
    #[validate(length(max = 100))]
    pub cover: String,
    #[validate(range(min = 0))]
    pub price: u32,
    pub status: model::SubjectStatus,
    #[validate(range(min = 1))]
    pub id: u32,
}
