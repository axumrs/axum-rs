use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::currency::Currency;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "pay_status")]
pub enum Status {
    #[default]
    Pending,
    Failed,
    Success,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "pay_method")]
pub enum Method {
    #[default]
    Online,
    QrCode,
    WechatAlipay,
    Pointer,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = pays, pk = id)]
pub struct Pay {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String, // CHAR(20) PRIMARY KEY ,

    /// 订单ID
    #[db(find_opt)]
    #[db(skip_update)]
    #[db(exists)]
    pub order_id: String, // CHAR(20) NOT NULL,

    /// 用户ID
    #[db(find_opt)]
    #[db(skip_update)]
    pub user_id: String, // CHAR(20) NOT NULL,

    /// 支付金额
    pub amount: Decimal, // DECIMAL(10,2) NOT NULL,
    /// 货币
    pub currency: Currency, // currency NOT NULL DEFAULT 'USDT',
    /// 支付工具的交易ID
    pub tx_id: String, // VARCHAR(255) NOT NULL DEFAULT '',
    /// 支付方式
    pub method: Method, // pay_method NOT NULL DEFAULT 'Online',

    /// 支付状态
    pub status: Status, // pay_status NOT NULL DEFAULT 'Pending',
    /// 是否管理员生成
    pub is_via_admin: bool, // BOOLEAN NOT NULL DEFAULT FALSE,
    /// 审核时间
    pub approved_time: DateTime<Local>, // TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08',
    /// 审核意见
    pub approved_opinion: String, // VARCHAR(255)

    /// 支付证明截图
    pub proof: String, //VARCHAR(255) NOT NULL DEFAULT '',
    /// 时间
    pub dateline: DateTime<Local>, // TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
}
