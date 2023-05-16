use serde::Deserialize;

#[derive(Deserialize)]
pub struct List {
    pub name: Option<String>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}
