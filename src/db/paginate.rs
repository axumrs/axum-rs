use serde::Serialize;

use crate::model::PaginateWith;

#[derive(Serialize)]
pub struct Paginate<T: Serialize> {
    pub total: u32,
    pub total_page: u32,
    pub page: u32,
    pub page_size: u32,
    pub data: Vec<T>,
}

impl<T: Serialize> Paginate<T> {
    pub fn new(total: u32, page: u32, page_size: u32, data: Vec<T>) -> Self {
        let total_page = (f64::ceil(total as f64 / page_size as f64)) as u32;
        Self {
            total,
            total_page,
            page,
            page_size,
            data,
        }
    }
    pub fn with(count: &(i64,), pw: &PaginateWith, data: Vec<T>) -> Self {
        Self::new(count.0 as u32, pw.page, pw.page_size, data)
    }
    pub fn last(&self) -> u32 {
        self.total_page - 1
    }
    pub fn has_prev(&self) -> bool {
        self.page > 0
    }
    pub fn has_next(&self) -> bool {
        self.page < self.last()
    }
}
