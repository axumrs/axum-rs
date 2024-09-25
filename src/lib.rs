pub mod config;
mod err;
pub mod interfaces;
pub mod mid;
pub mod model;
pub mod service;
mod state;
pub mod utils;

pub use err::Error;
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
