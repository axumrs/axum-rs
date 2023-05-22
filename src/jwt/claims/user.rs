use serde::{Deserialize, Serialize};

use crate::model;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserClaimsData {
    pub id: u32,
    pub email: String,
    pub nickname: String,
    pub status: model::UserStatus,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub types: model::UserTypes,
    pub sub_exp: chrono::DateTime<chrono::Local>,
    pub points: u32,
    pub allow_device_num: u8,
    pub available_device_num: u8,
}
