use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Add {
    #[validate(length(min = 1, max = 100))]
    pub name: String, // VARCHAR(100) NOT NULL,

    /// 是否专题
    pub is_subject: bool, // BOOLEAN NOT NULL DEFAULT FALSE,

    /// 目标ID
    pub target_id: String, // CHAR(20) NOT NULL,
    /// 时效(天)
    pub duration: i16, // SMALLINT NOT NULL DEFAULT 0,
    /// 价格
    pub price: Decimal, // DECIMAL(10,2) NOT NULL,
    /// 封面
    #[validate(length(max = 100))]
    pub cover: String, // VARCHAR(100) NOT NULL DEFAULT '',
    /// 是否允许积分兑换
    pub allow_pointer: bool, // BOOLEAN NOT NULL DEFAULT FALSE,
    /// 普通用户折扣
    pub normal_discount: i16, // SMALLINT NOT NULL DEFAULT 0,
    /// 订阅用户折扣
    pub sub_discount: i16, // SMALLINT NOT NULL DEFAULT 0,
    /// 年费用户折扣
    pub yearly_sub_discount: i16, // SMALLINT NOT NULL DEFAULT 0,

    /// 是否下架
    pub is_off: bool, // BOOLEAN NOT NULL DEFAULT FALSE
    pub desc: String,
    pub pin: i32,
}

#[derive(Deserialize, Validate)]
pub struct Edit {
    #[serde(flatten)]
    pub base: Add,

    #[validate(length(min = 20, max = 20))]
    pub id: String,
}
