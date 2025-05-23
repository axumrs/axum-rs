pub mod api;
pub mod captcha;
pub mod config;
mod err;
pub mod form;
pub mod interfaces;
pub mod mail;
pub mod mid;
pub mod model;
mod resp;
pub mod service;
mod state;
pub mod tron;
pub mod utils;

pub use err::Error;
pub use resp::*;
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
