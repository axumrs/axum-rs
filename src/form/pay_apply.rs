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
