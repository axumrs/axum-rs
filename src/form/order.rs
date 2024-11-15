use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

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
