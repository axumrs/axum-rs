use serde::Deserialize;

use crate::{model, Error, Result};

#[derive(Deserialize)]
pub struct Web {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Mysql {
    pub max_conns: u32,
    pub dsn: String,
}

#[derive(Deserialize, Clone)]
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
pub struct User {
    pub register_default_status: model::UserStatus,
    pub redis_prefix: String,
    pub redis_allow_drive_prefix: String,
    pub redis_jwt_exp_prefix: String,
    pub redis_online_prefix: String,
}
#[derive(Deserialize)]
pub struct ProtectedTopic {
    pub redis_prefix: String,
    pub redis_expired: u8,
    pub max_paragraph_num: u8,
    pub min_content_paragraph_num: u8,
    pub guest_captcha: model::ProtectedTopic2WebDetailCaptchaType,
    pub normal_user_captcha: model::ProtectedTopic2WebDetailCaptchaType,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub mysql: Mysql,
    pub admin_jwt: Jwt,
    pub user_jwt: Jwt,
    pub redis: Redis,
    pub hcaptcha: HCaptcha,
    pub recaptcha: ReCaptcha,
    pub users: User,
    pub protected_topic: ProtectedTopic,
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
