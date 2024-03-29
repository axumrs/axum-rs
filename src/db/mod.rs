pub mod admin;
mod del_restore;
pub mod order;
mod paginate;
pub mod pay;
pub mod pay_apply;
pub mod subject;
pub mod tag;
pub mod topic;
pub mod user;
pub mod user_login_log;
pub mod user_purchased_service;
pub mod user_purchased_subject;
pub mod user_read_history;

pub const DEFAULT_PAGE_SIZE: u32 = 30;
pub const MAX_PAGE_SIZE: u32 = 500;

pub use del_restore::invoke as del_or_restore;
pub use del_restore::PrimaryKey as DelOrRestorePrimaryKey;
pub use paginate::Paginate;
