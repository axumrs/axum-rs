mod admin;
mod order;
mod pay;
mod pay_apply;
mod state;
mod subject;
mod tag;
mod topic;
mod user;
mod user_check_in;
mod user_login_log;
mod user_purchased_service;

pub use admin::*;
pub use order::*;
pub use pay::*;
pub use pay_apply::*;
pub use state::*;
pub use subject::*;
pub use tag::*;
pub use topic::*;
pub use user::*;
pub use user_check_in::*;
pub use user_login_log::*;
pub use user_purchased_service::*;

pub struct PaginateWith {
    pub page: u32,
    pub page_size: u32,
}
