use serde::Deserialize;

pub mod admin;
pub mod auth;
pub mod order;
pub mod pay;
pub mod subject;
pub mod tag;
pub mod topic;
pub mod user;

#[derive(Deserialize)]
pub struct PaginateForm {
    pub page: u32,
    pub page_size: u32,
}
