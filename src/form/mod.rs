use serde::{Deserialize, Serialize};

pub mod auth;
pub mod user;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PageQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl PageQuery {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(30)
    }
}
