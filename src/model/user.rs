use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
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
