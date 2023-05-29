use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Get {
    pub protect_ids: Vec<String>,
    #[validate(length(min = 1))]
    pub response: String,
}
