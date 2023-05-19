use serde::Deserialize;

pub mod admin;
pub mod subject;
pub mod tag;
pub mod topic;

#[derive(Deserialize)]
pub struct PaginateForm {
    pub page: u32,
    pub page_size: u32,
}
