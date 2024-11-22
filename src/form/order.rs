use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

use crate::model::{self, currency::Currency, pay::Method};

#[derive(Deserialize, Validate)]
pub struct Create {
    pub services: Vec<ServiceForCreate>,
    pub amount: Decimal,
    pub actual_amount: Decimal,
}

#[derive(Deserialize, Validate)]
pub struct ServiceForCreate {
    #[validate(length(min = 20, max = 20))]
    pub id: String,
    #[validate(range(min = 1, max = 96))]
    pub num: i16,
}

#[derive(Deserialize)]
pub struct ListForAdmin {
    #[serde(flatten)]
    pub pq: super::PageQueryStr,
}

#[derive(Deserialize, Validate)]
pub struct AddForAdmin {
    #[validate(length(min = 20, max = 20))]
    pub user_id: String,
    pub snap: Vec<model::order::OrderSnapshot>,
    pub amount: Decimal,
    pub currency: Currency,
    pub method: Method,
    pub tx_id: String,
    pub is_via_admin: bool,
    pub approved_opinion: String,
    pub proof: String,
}
