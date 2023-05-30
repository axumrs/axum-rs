use serde::{Deserialize, Serialize};

use crate::{order_meta, Result};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[repr(u8)]
pub enum OrderStatus {
    #[default]
    Pending = 0,
    Finished = 1,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct Order {
    pub id: u64,
    pub user_id: u32,
    pub price: u32,
    pub status: OrderStatus,
    pub code: String,
    pub full_code: String,
    pub order_num: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub pay_id: u64,
    pub is_del: bool,
}

impl Order {
    pub fn new(user_id: u32, price: u32) -> Result<Self> {
        let order_num = order_meta::order::number();
        let (full_code, code) = order_meta::order::code(&order_num)?;
        Ok(Self {
            user_id,
            price: price * 100,
            dateline: chrono::Local::now(),
            order_num,
            full_code,
            code,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct OrderSnap {
    pub order_id: u64,
    pub snap: String,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct OrderFull {
    pub id: u64,
    pub user_id: u32,
    pub price: u32,
    pub status: OrderStatus,
    pub code: String,
    pub full_code: String,
    pub order_num: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub pay_id: u64,
    pub is_del: bool,
    pub snap: String,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct OrderWithUser {
    pub id: u64,
    pub user_id: u32,
    pub price: u32,
    pub status: OrderStatus,
    pub code: String,
    pub full_code: String,
    pub order_num: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub pay_id: u64,
    pub is_del: bool,
    pub email: String,
    pub nickname: String,
}
#[derive(Default)]
pub struct OrderListWith {
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub user_id: Option<u32>,
    pub pay_id: Option<u64>,
    pub code: Option<String>,
    pub order_num: Option<String>,
    pub status: Option<OrderStatus>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

// [{"type":"订阅","service":"成为尊贵的订阅用户","serviceID":1,"price":1,"number":1,"idx":1,"id":"订阅成为尊贵的订阅用户1"}]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OrderSnapItem {
    #[serde(rename = "type")]
    pub types: String,
    pub service: String,
    #[serde(rename = "serviceID")]
    pub server_id: u32,
    pub price: u32,
    pub number: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct OrderFullWithUser {
    pub id: u64,
    pub user_id: u32,
    pub price: u32,
    pub status: OrderStatus,
    pub code: String,
    pub full_code: String,
    pub order_num: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub pay_id: u64,
    pub is_del: bool,
    pub snap: String,
    pub email: String,
    pub nickname: String,
}
