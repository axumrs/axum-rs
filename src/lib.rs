mod err;
pub mod model;
pub mod service;
pub mod utils;

pub use err::Error;

pub type Result<T> = std::result::Result<T, crate::Error>;
