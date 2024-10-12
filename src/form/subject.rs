use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

use crate::model::{self, subject::Status};

#[derive(Deserialize)]
pub struct ListForAdmin {
    #[serde(flatten)]
    pub pq: super::PageQueryStr,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub status: Option<model::subject::Status>,
    pub is_del: Option<String>,
}

impl ListForAdmin {
    pub fn is_del(&self) -> Option<bool> {
        if let Some(ref v) = self.is_del {
            Some(v == "1")
        } else {
            None
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct Add {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub slug: String,

    #[validate(length(min = 1, max = 255))]
    pub summary: String,

    #[validate(length(min = 0, max = 100))]
    pub cover: String,

    pub status: Status,

    pub price: Decimal,
    pub pin: i32,
}

#[derive(Deserialize, Validate)]
pub struct Edit {
    #[validate(length(min = 20, max = 20))]
    pub id: String,

    #[serde(flatten)]
    pub base: Add,
}

#[derive(Deserialize)]
pub struct RealDel {
    pub real: Option<bool>,
}
