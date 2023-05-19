pub mod admin_api;
mod config;
pub mod db;
mod err;
pub mod form;
pub mod handler_helper;
pub mod md;
pub mod middleware;
pub mod model;
pub mod password;
pub mod rdb;
mod resp;
pub mod web_api;

pub use crate::config::*;
pub use err::Error;
pub use err::Kind as ErrorKind;
pub use resp::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
