use serde::Deserialize;

pub mod admin;
pub mod auth;
pub mod order;
pub mod pay;
pub mod pay_apply;
pub mod protected_content;
pub mod subject;
pub mod tag;
pub mod topic;
pub mod user;
pub mod user_purchased_subject;

#[derive(Deserialize)]
pub struct PaginateForm {
    pub page: u32,
    pub page_size: u32,
}
