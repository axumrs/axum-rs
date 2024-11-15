use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Paginate<T: Serialize> {
    pub total: u32,
    pub total_page: u32,
    pub page: u32,
    pub page_size: u32,
    pub data: Vec<T>,
}

impl<T: Serialize + DeserializeOwned> Paginate<T> {
    pub fn new(total: u32, page: u32, page_size: u32, data: Vec<T>) -> Self {
        let total_page = (total as f64 / page_size as f64).ceil() as u32;
        Self {
            total,
            page,
            page_size,
            data,
            total_page,
        }
    }

    pub fn quick(count: (i64,), page: u32, page_size: u32, data: Vec<T>) -> Self {
        Self::new(count.0 as u32, page, page_size, data)
    }
}
