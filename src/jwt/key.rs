use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::config;

pub struct Key {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Key {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
    pub fn from_cfg(cfg: &config::Jwt) -> Self {
        Self::new(&cfg.secret_key.as_bytes())
    }
}
