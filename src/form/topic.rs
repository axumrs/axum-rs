use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetProtectedContent {
    #[validate(length(min = 1))]
    pub ids: Vec<String>,

    #[validate(length(min = 20, max = 20))]
    pub topic_id: String,

    #[validate(length(min = 6))]
    pub captcha: String,
}
