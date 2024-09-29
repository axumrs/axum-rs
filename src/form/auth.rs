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
pub struct AdminLoginForm {
    #[validate(length(min = 3))]
    pub username: String,

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

    /// 邀请码
    pub invite: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct RegisterSendCodeForm {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub captcha: String,
}
