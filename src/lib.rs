mod assets;
mod config;
mod err;
pub mod handler;
pub mod log;
mod state;
pub mod template;

pub use crate::config::*;
pub use assets::Asset;
pub use err::*;
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
