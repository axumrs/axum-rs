use std::ops::Deref;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct Base {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub content: String,
    #[validate(length(min = 1, max = 255))]
    pub url: String,
    #[validate(length(max = 255))]
    pub img: String,
}

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct Add {
    #[serde(flatten)]
    pub inner: Base,
}

impl Deref for Add {
    type Target = Base;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct Edit {
    #[validate(length(min = 20, max = 20))]
    pub id: String,
    #[serde(flatten)]
    pub inner: Base,
}

impl Deref for Edit {
    type Target = Base;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Deserialize)]
pub struct ListForAdmin {
    #[serde(flatten)]
    pub pq: super::PageQueryStr,
    pub name: Option<String>,
}
