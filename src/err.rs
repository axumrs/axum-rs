use axum::response::IntoResponse;

use crate::Response;

#[derive(Debug)]
pub enum Kind {
    Database,
    AlreadyExists,
    Config,
    NotFound,
}
#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub kind: Kind,
}

impl Error {
    pub fn new(message: String, cause: Option<Box<dyn std::error::Error>>, kind: Kind) -> Self {
        Self {
            message,
            cause,
            kind,
        }
    }
    pub fn with_cause(cause: Box<dyn std::error::Error>, kind: Kind) -> Self {
        Self::new(cause.to_string(), Some(cause), kind)
    }
    pub fn from_str(msg: &str, kind: Kind) -> Self {
        Self::new(msg.to_string(), None, kind)
    }
    pub fn already_exists(msg: &str) -> Self {
        Self::from_str(msg, Kind::AlreadyExists)
    }
    pub fn not_found(msg: &str) -> Self {
        Self::from_str(msg, Kind::NotFound)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::with_cause(Box::new(e), Kind::Database)
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Self::with_cause(Box::new(e), Kind::Config)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::<()>::err(&self).to_json().into_response()
    }
}