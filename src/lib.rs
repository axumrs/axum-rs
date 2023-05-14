mod config;
pub mod db;
mod err;
pub mod model;
mod resp;

pub use crate::config::*;
pub use err::Error;
pub use err::Kind as ErrorKind;
pub use resp::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
