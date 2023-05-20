use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AdminLogin {
    #[validate(length(max = 50))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1))]
    pub response: String,
}
