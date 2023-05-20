use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    config,
    jwt::{AuthBody, Key},
    Error, Result,
};

#[derive(Serialize, Deserialize)]
pub struct Claims<T> {
    /// 主题
    pub sub: String,
    /// 签发机构
    pub iss: String,
    /// 过期时间
    pub exp: i64,
    /// 签发时间
    pub iat: i64,

    /// 数据
    pub data: T,
}

impl<T: Serialize + DeserializeOwned> Claims<T> {
    pub fn new(sub: String, exp: i64, iat: i64, data: T) -> Self {
        Self {
            sub,
            iss: "axum.rs".to_string(),
            exp,
            iat,
            data,
        }
    }
    pub fn with_exp(sub: String, exp_minutes: u32, data: T) -> Self {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::minutes(exp_minutes as i64);

        let iat = now.timestamp();
        let exp = exp.timestamp();
        Self::new(sub, exp, iat, data)
    }
    pub fn from_cfg(cfg: &config::Jwt, data: T) -> Self {
        Self::with_exp(cfg.sub.clone(), cfg.expired, data)
    }
    pub fn token(&self, key: &Key) -> Result<AuthBody> {
        let token = encode(&Header::default(), &self, &key.encoding).map_err(Error::from)?;
        Ok(AuthBody::new(token))
    }
    pub fn from_token(token: &str, key: &Key) -> Result<Self> {
        let token_data =
            decode(token, &key.decoding, &Validation::default()).map_err(Error::from)?;
        Ok(token_data.claims)
    }
}
