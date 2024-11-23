use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

use crate::model::{currency::Currency, pay::Method};

#[derive(Deserialize, Validate)]
pub struct UserPay {
    #[validate(length(min = 20, max = 20))]
    pub order_id: String,

    pub amount: Decimal,
    pub currency: Currency,
    pub method: Method,
    #[validate(length(min = 64, max = 64))]
    pub tx_id: String,

    pub re_pay: bool,
}

#[derive(Deserialize, Validate)]
pub struct UserConfirm {
    #[validate(length(min = 20, max = 20))]
    pub order_id: String,
    pub currency: Currency,
    #[validate(length(min = 20, max = 20))]
    pub pay_id: String,
}
