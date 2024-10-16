use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 6))]
    pub new_password: String,
    #[validate(length(min = 6))]
    pub re_password: String,
}
