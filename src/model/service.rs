use axum_rs_derive::Db;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = services, pk = id)]
pub struct Service {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String, // CHAR(20) PRIMARY KEY,

    #[db(list_opt)]
    #[db(list_opt_like)]
    #[db(exists)]
    pub name: String, // VARCHAR(100) NOT NULL,

    /// 是否专题
    #[db(list_opt)]
    #[db(find_opt)]
    pub is_subject: bool, // BOOLEAN NOT NULL DEFAULT FALSE,

    /// 目标ID
    #[db(exists)]
    #[db(find_opt)]
    pub target_id: String, // CHAR(20) NOT NULL,
    /// 时效(天)
    pub duration: i16, // SMALLINT NOT NULL DEFAULT 0,
    /// 价格
    pub price: Decimal, // DECIMAL(10,2) NOT NULL,
    /// 封面
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
    #[db(list_opt)]
    pub is_off: bool, // BOOLEAN NOT NULL DEFAULT FALSE
    pub desc: String,
    pub pin: i32,
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let s = super::Service::default();
        println!("{}", serde_json::json!(&s));
    }
}
