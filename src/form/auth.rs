use serde::Deserialize;
use validator::Validate;

use crate::model;

use super::user;

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 6))]
    pub captcha: String,
}
#[derive(Deserialize, Validate)]
pub struct AdminLoginForm {
    #[validate(length(min = 3, max = 50))]
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

    /// 邀请码
    pub invite: Option<String>,

    #[validate(length(min = 6))]
    pub captcha: String,
}

#[derive(Deserialize, Validate)]
pub struct SendCodeForm {
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,

    #[validate(length(min = 6))]
    pub captcha: String,

    pub kind: model::activation_code::Kind,
}

#[derive(Deserialize, Validate)]
pub struct ActiveForm {
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,

    #[validate(length(min = 6))]
    pub activation_code: String,

    #[validate(length(min = 6))]
    pub captcha: String,

    pub kind: model::activation_code::Kind,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordForm {
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,

    #[validate(length(min = 6))]
    pub activation_code: String,

    #[validate(length(min = 6))]
    pub captcha: String,

    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 6))]
    pub re_password: String,
}
