use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Paginate<T: Serialize> {
    pub total: u32,
    pub total_page: u32,
    pub page: u32,
    pub page_size: u32,
    pub data: Vec<T>,
}
