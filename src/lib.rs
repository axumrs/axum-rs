mod err;

pub use err::Error;

pub type Result<T> = std::result::Result<T, crate::Error>;
