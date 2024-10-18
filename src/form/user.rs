use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

use crate::{model, utils};

#[derive(Deserialize, Validate)]
pub struct AddForm {
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,

    #[validate(length(min = 3, max = 30))]
    pub nickname: String,

    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 6))]
    pub re_password: String,
}

#[derive(Deserialize, Validate)]
pub struct AddForAdmin {
    #[serde(flatten)]
    pub base: AddForm,

    pub status: model::user::Status,
    pub kind: model::user::Kind,
    pub sub_exp: Option<String>,
    pub points: Decimal,
    pub allow_device_num: i16,
    pub session_exp: i16,
}
impl AddForAdmin {
    pub fn sub_exp(&self) -> DateTime<Local> {
        let default_ts = utils::dt::parse("1970-01-01 00:00:00").unwrap_or_default();
        match self.sub_exp {
            Some(ref v) => utils::dt::parse(v).unwrap_or(default_ts),
            None => default_ts,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct EditForAdmin {
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,

    #[validate(length(min = 3, max = 30))]
    pub nickname: String,

    pub status: model::user::Status,
    pub kind: model::user::Kind,
    pub sub_exp: Option<String>,
    pub points: Decimal,
    pub allow_device_num: i16,
    pub session_exp: i16,

    #[validate(length(min = 20, max = 20))]
    pub id: String,
}

impl EditForAdmin {
    pub fn sub_exp(&self) -> DateTime<Local> {
        let default_ts = utils::dt::parse("1970-01-01 00:00:00").unwrap_or_default();
        match self.sub_exp {
            Some(ref v) => utils::dt::parse(v).unwrap_or(default_ts),
            None => default_ts,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct ListForAdmin {
    #[serde(flatten)]
    pub pq: super::PageQueryStr,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub status: Option<model::user::Status>,
    pub kind: Option<model::user::Kind>,
}
