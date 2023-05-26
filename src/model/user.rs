use serde::{Deserialize, Serialize};

use crate::uap;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[repr(u8)]
pub enum UserStatus {
    /// 待激活
    #[default]
    Pending = 0,
    /// 正常，已激活
    Actived = 1,
    /// 被冻结
    Freezed = 2,
}
#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum UserTypes {
    /// 普通用户
    #[default]
    Normal = 0,
    /// 订阅用户
    Subscriber = 1,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub nickname: String,
    pub password: String,
    pub status: UserStatus,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub types: UserTypes,
    pub sub_exp: chrono::DateTime<chrono::Local>,
    pub points: u32,
    pub allow_device_num: u8,
    pub jwt_exp: u8,
    pub is_del: bool,
}
#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct UserEdit2Admin {
    pub id: u32,
    pub email: String,
    pub nickname: String,
    pub password: Option<String>,
    pub status: UserStatus,
    pub types: UserTypes,
    pub sub_exp: chrono::DateTime<chrono::Local>,
    pub allow_device_num: u8,
    pub jwt_exp: u8,
    pub points: u32,
}

pub enum UserFindBy<'a> {
    ID(u32),
    Email(&'a str),
    Nickname(&'a str),
}

#[derive(Default)]
pub struct UserListWith {
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub status: Option<UserStatus>,
    pub types: Option<UserTypes>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}
#[derive(Default)]
pub struct UserLoginMeta {
    pub email: String,
    pub password: String,
    pub ip: String,
    pub uai: uap::UserAgentInfo,
    pub ua: String,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct UserSubscribeInfo {
    pub types: UserTypes,
    pub sub_exp: chrono::DateTime<chrono::Local>,
}
