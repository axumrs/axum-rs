use axum::response::IntoResponse;

use crate::Response;

#[derive(Debug)]
pub enum Kind {
    Database,
    AlreadyExists,
    Config,
    NotFound,
    Bcrypt,
    Redis,
    Reqwest,
    Serde,
    Captcha,
    Jwt,
    Validator,
    UserAgent,
    NoAvailableDevice,
    Base16,
    Chrono,
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
    pub fn captcha_failed() -> Self {
        Self::from_str("人机验证失败", Kind::Captcha)
    }
    pub fn no_available_device() -> Self {
        Self::from_str(
            "没有足够的登录配额，请退出其它已登录设备后再重试",
            Kind::NoAvailableDevice,
        )
    }
    pub fn no_token() -> Self {
        Self::from_str("未提供有效凭证", Kind::Jwt)
    }
    pub fn code(&self) -> i32 {
        match &self.kind {
            &Kind::Jwt => 9527,
            &Kind::Validator => 9528,
            _ => -1,
        }
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

impl From<bcrypt::BcryptError> for Error {
    fn from(e: bcrypt::BcryptError) -> Self {
        Self::with_cause(Box::new(e), Kind::Bcrypt)
    }
}

impl From<redis::RedisError> for Error {
    fn from(e: redis::RedisError) -> Self {
        Self::with_cause(Box::new(e), Kind::Redis)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::with_cause(Box::new(e), Kind::Reqwest)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::with_cause(Box::new(e), Kind::Serde)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::with_cause(Box::new(e), Kind::Jwt)
    }
}

impl From<validator::ValidationError> for Error {
    fn from(e: validator::ValidationError) -> Self {
        Self::with_cause(Box::new(e), Kind::Validator)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Self::with_cause(Box::new(e), Kind::Validator)
    }
}

impl From<uaparser::Error> for Error {
    fn from(e: uaparser::Error) -> Self {
        Self::from_str(&e.to_string(), Kind::UserAgent)
    }
}

impl From<base16ct::Error> for Error {
    fn from(e: base16ct::Error) -> Self {
        Self::from_str(&e.to_string(), Kind::Base16)
    }
}
impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Self::with_cause(Box::new(e), Kind::Chrono)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::<()>::err_with_code(self.code(), &self)
            .to_json()
            .into_response()
    }
}
