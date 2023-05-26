use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum PayCurrency {
    /// USDT TRC-20
    #[default]
    USDT = 0,
    /// 人民币
    CNY = 1,
}
#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum PayTypes {
    /// TronLink 钱包在线支付
    #[default]
    TronLink = 0,
    /// USDT 手动转账
    USDT = 1,
    /// 支付宝手动转账
    Alipay = 2,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum PayStatus {
    /// 待支付
    #[default]
    Pending = 0,
    /// 待确认
    UnConfirmed = 1,
    /// 支付完成
    Finished = 2,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct Pay {
    pub id: u64,
    pub order_id: u64,
    pub user_id: u32,
    pub price: u32,
    pub currency: PayCurrency,
    pub types: PayTypes,
    pub tx_id: String,
    pub status: PayStatus,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
}
