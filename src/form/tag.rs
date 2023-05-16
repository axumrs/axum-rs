use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct List {
    pub name: Option<String>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Deserialize, Validate)]
pub struct Create {
    #[validate(length(max = 100))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct Update {
    #[validate(range(min = 1))]
    pub id: u32,
    #[validate(length(max = 100))]
    pub name: String,
}
