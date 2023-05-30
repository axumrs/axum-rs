use serde::Deserialize;

#[derive(Deserialize)]
pub struct List {
    pub user_id: Option<u32>,
    pub page: u32,
    pub page_size: u32,
}
