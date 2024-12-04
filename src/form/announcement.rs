use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Add {
    #[validate(length(min = 1, max = 255))]
    pub title: String,

    #[validate(length(min = 1))]
    pub content: String,
}
#[derive(Deserialize, Validate)]
pub struct Edit {
    #[validate(length(min = 20, max = 20))]
    pub id: String,
    #[serde(flatten)]
    pub base: Add,
}

#[derive(Deserialize)]
pub struct ListForAdmin {
    #[serde(flatten)]
    pub pq: super::PageQueryStr,
    pub title: Option<String>,
}
