use serde::{Deserialize, Serialize};

pub mod announcement;
pub mod auth;
pub mod order;
pub mod pay;
pub mod profile;
pub mod service;
pub mod subject;
pub mod tag;
pub mod topic;
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PageQueryStr {
    pub page: Option<String>,
    pub page_size: Option<String>,
}

impl PageQueryStr {
    pub fn page(&self) -> u32 {
        self.page
            .clone()
            .unwrap_or("0".into())
            .parse()
            .unwrap_or_default()
    }

    pub fn page_size(&self) -> u32 {
        self.page_size
            .clone()
            .unwrap_or("30".into())
            .parse()
            .unwrap_or(30)
    }

    pub fn page_to_bind(&self) -> i64 {
        self.page() as i64
    }

    pub fn page_size_to_bind(&self) -> i64 {
        self.page_size() as i64
    }

    pub fn offset_to_bind(&self) -> i64 {
        self.page_to_bind() * self.page_size_to_bind()
    }
}

#[derive(Deserialize)]
pub struct ListAll {
    pub limit: Option<i64>,
    pub has_price: Option<bool>,
}
