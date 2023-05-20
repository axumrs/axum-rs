use serde::Deserialize;

use crate::{Error, Result};

#[derive(Deserialize)]
pub struct Web {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Mysql {
    pub max_conns: u32,
    pub dsn: String,
}

#[derive(Deserialize)]
pub struct Jwt {
    pub secret_key: String,
    pub expired: u32,
    pub sub: String,
}

#[derive(Deserialize)]
pub struct Redis {
    pub dsn: String,
    pub prefix: String,
}

#[derive(Deserialize)]
pub struct HCaptcha {
    pub site_key: String,
    pub secret_key: String,
}
#[derive(Deserialize)]
pub struct ReCaptcha {
    pub site_key: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub mysql: Mysql,
    pub admin_jwt: Jwt,
    pub redis: Redis,
    pub hcaptcha: HCaptcha,
    pub recaptcha: ReCaptcha,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(Error::from)?
            .try_deserialize()
            .map_err(Error::from)
    }
}
