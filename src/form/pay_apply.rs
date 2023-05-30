use serde::Deserialize;
use validator::Validate;

use crate::model;

#[derive(Deserialize, Validate)]
pub struct Create {
    pub order_id: u64,
    pub price: u32,
    pub currency: model::PayCurrency,
    pub types: model::PayTypes,
    #[validate(length(max = 255))]
    pub tx_id: String,
    #[validate(length(max = 255))]
    pub img: String,
}
#[derive(Deserialize, Validate)]
pub struct AdminReject {
    pub id: u64,
    #[validate(length(max = 255))]
    pub reason: String,
}
#[derive(Deserialize, Validate)]
pub struct AdminAccept {
    pub id: u64,
    #[validate(length(max = 255))]
    pub reason: String,
}
