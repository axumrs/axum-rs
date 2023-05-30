use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum UserPurchasedServiceType {
    #[default]
    Subscriber = 0,
    Subject = 1,
}
#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum UserPurchasedServiceStatus {
    #[default]
    Pending = 0,
    Finished = 1,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct UserPurchasedService {
    pub id: u64,
    pub order_id: u64,
    pub user_id: u32,
    pub service_id: u32,
    pub service_type: UserPurchasedServiceType,
    pub server_num: u32,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub status: UserPurchasedServiceStatus,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct UserPurchasedServiceFull {
    // subject
    pub subject_id: Option<u32>,
    pub subject_name: Option<String>,
    pub subject_slug: Option<String>,
    pub subject_summary: Option<String>,
    pub subject_is_del: Option<bool>,
    pub subject_cover: Option<String>,
    pub subject_status: Option<super::SubjectStatus>,
    pub subject_price: Option<u32>,

    // user
    pub email: String,
    pub nickname: String,

    // purchased
    pub id: u64,
    pub order_id: u64,
    pub user_id: u32,
    pub service_id: u32,
    pub service_type: UserPurchasedServiceType,
    pub server_num: u32,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub status: UserPurchasedServiceStatus,

    // order
    pub order_num: String,
}

pub struct UserPurchasedServiceFullListWith {
    pub user_id: Option<u32>,
    pub pw: super::PaginateWith,
}
