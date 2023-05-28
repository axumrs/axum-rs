use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct UserPurchasedSubject {
    // subject
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub is_del: bool,
    pub cover: String,
    pub status: super::SubjectStatus,
    pub price: u32,

    // user
    pub email: String,
    pub nickname: String,

    // purchased
    pub purchased_id: u64,
    pub order_id: u64,
    pub user_id: u32,
    pub service_id: u32,
    pub service_type: super::UserPurchasedServiceType,
    pub server_num: u32,
    pub purchased_dateline: chrono::DateTime<chrono::Local>,
    pub purchased_status: super::UserPurchasedServiceStatus,
}

pub enum UserPurchasedSubjectFindBy<'a> {
    PurchasedID(u64),
    PurchasedIDWithUser { id: u64, user_id: u32 },
    SubjectID { subject_id: u32, user_id: u32 },
    Subject { slug: &'a str, user_id: u32 },
}

pub struct UserPurchasedSubjectListWith {
    pub user_id: Option<u32>,
    pub subject_id: Option<u32>,
    pub subject_slug: Option<String>,
    pub purchased_id: Option<u64>,
    pub subject_name: Option<String>,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub is_del: Option<bool>,
    pub pw: super::PaginateWith,
}
