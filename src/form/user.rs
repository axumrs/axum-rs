use serde::Deserialize;
use validator::Validate;

use crate::model;

#[derive(Deserialize, Validate)]
pub struct Create {
    #[validate(length(max = 255))]
    pub email: String,
    #[validate(length(max = 30))]
    pub nickname: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub status: Option<model::UserStatus>,
    pub types: Option<model::UserTypes>,
    #[serde(deserialize_with = "crate::serde_with::chrono::deserialize")]
    pub sub_exp: chrono::DateTime<chrono::Local>,
    #[validate(range(min = "std::u32::MIN", max = "std::u32::MAX"))]
    pub points: u32,
    #[validate(range(min = 1, max = 3))]
    pub allow_device_num: u8,
    #[validate(range(min = 0, max = 120))]
    pub jwt_exp: u8,
}

#[derive(Deserialize)]
pub struct List {
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub status: Option<model::UserStatus>,
    pub types: Option<model::UserTypes>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Deserialize, Validate)]
pub struct Update {
    pub id: u32,
    #[validate(length(max = 255))]
    pub email: String,
    #[validate(length(max = 30))]
    pub nickname: String,
    #[validate(length(min = 6))]
    pub password: Option<String>,
    pub status: model::UserStatus,
    pub types: model::UserTypes,
    #[serde(deserialize_with = "crate::serde_with::chrono::deserialize")]
    pub sub_exp: chrono::DateTime<chrono::Local>,
    #[validate(range(min = "std::u32::MIN", max = "std::u32::MAX"))]
    pub points: u32,
    #[validate(range(min = 1, max = 3))]
    pub allow_device_num: u8,
    #[validate(range(min = 0, max = 120))]
    pub jwt_exp: u8,
}

#[derive(Deserialize, Validate)]
pub struct Profile {
    pub id: u32,
    #[validate(length(max = 255))]
    pub email: String,
    #[validate(length(max = 30))]
    pub nickname: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(range(min = 1, max = 3))]
    pub allow_device_num: u8,
    #[validate(range(min = 0, max = 120))]
    pub jwt_exp: u8,
}

#[derive(Deserialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 6))]
    pub new_password: String,

    #[validate(must_match(other = "new_password", message = "两次输入的密码不一致"))]
    pub re_password: String,
}
