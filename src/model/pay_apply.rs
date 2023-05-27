use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum PayApplyStatus {
    /// 待审核
    #[default]
    Pending = 0,
    /// 拒绝
    Reject = 1,
    /// 通过
    Finished = 2,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct PayApply {
    pub id: u64,
    pub order_id: u64,
    pub user_id: u32,
    pub price: u32,
    pub currency: super::PayCurrency,
    pub types: super::PayTypes,
    pub tx_id: String,
    pub status: PayApplyStatus,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
    pub img: String,
    pub process_dateline: chrono::DateTime<chrono::Local>,
    pub reason: String,
}

pub enum PayApplyFindBy {
    Owner { order_id: u64, user_id: u32 },
    ID(u64),
}
