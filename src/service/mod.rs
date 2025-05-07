pub mod activation_code;
pub mod admin;
pub mod order;
pub mod pay;
pub mod promotion;
pub mod subject;
pub mod tag;
pub mod topic;
pub mod topic_section;
pub mod topic_tag;
pub mod user;

use sqlx::{Postgres, Transaction};
pub type Tx<'a> = Transaction<'a, Postgres>;
