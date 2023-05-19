use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Create {
    #[validate(length(max = 50))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct Update {
    #[validate(range(min = 1))]
    pub id: u32,
    #[validate(length(max = 50))]
    pub username: String,
    pub password: Option<String>,
}
