use serde::{Deserialize, Serialize};

use crate::{model, uap};

#[derive(Deserialize, Serialize, Debug, Default)]
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
    pub login_id: u64,
    pub online_id: String,
    pub ip: String,
    pub uai: uap::UserAgentInfo,
}
