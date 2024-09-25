use serde::Deserialize;
use validator::Validate;

use super::user;

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 6))]
    pub captcha: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterForm {
    #[serde(flatten)]
    pub user: user::AddForm,

    #[validate(length(min = 6))]
    pub captcha: String,
}
