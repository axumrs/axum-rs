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
