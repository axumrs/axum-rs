use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub const DEFAULT_PAGE_SIZE: u32 = 30;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub page: u32,
    pub page_size: u32,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl PaginationQuery {
    pub fn page(&self) -> i32 {
        self.page as i32
    }
    pub fn page_size(&self) -> i32 {
        self.page_size as i32
    }
    pub fn limit(&self) -> i32 {
        self.page() * self.page_size()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination<T> {
    #[serde(flatten)]
    pub pq: PaginationQuery,
    pub total: u32,
    pub total_page: u32,
    pub data: Vec<T>,
}

impl<T: Serialize + DeserializeOwned> Pagination<T> {
    pub fn with_page_size(page: u32, page_size: u32, total: u32, data: Vec<T>) -> Self {
        let total_page = f64::ceil(total as f64 / page_size as f64) as u32;
        Self {
            pq: PaginationQuery { page, page_size },
            total,
            total_page,
            data,
        }
    }
    pub fn new(page: u32, total: u32, data: Vec<T>) -> Self {
        Self::with_page_size(page, DEFAULT_PAGE_SIZE, total, data)
    }
    pub fn quick(pq: &PaginationQuery, total: i64, data: Vec<T>) -> Self {
        Self::with_page_size(pq.page, pq.page_size, total as u32, data)
    }
}
