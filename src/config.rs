use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub dsn: String,
    pub max_conns: u32,
}

#[derive(Debug, Deserialize)]
pub struct SessionConfig {
    pub secret_key: String,
    pub default_timeout: u32,
    pub max_timeout: u32,
}

#[derive(Debug, Deserialize)]
pub struct MailConfig {
    pub name: String,
    pub smtp: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ProtectedContentConfig {
    pub max_sections: u8,
    pub min_sections: u8,
    pub guest_captcha: CaptchaKind,
    pub user_captcha: CaptchaKind,
}

#[derive(Debug, Deserialize)]
pub enum CaptchaKind {
    HCaptcha,
    Turnstile,
}

#[derive(Debug, Deserialize)]
pub struct CaptchaItemConfig {
    pub secret_key: String,
    pub validation_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CaptchaConfig {
    pub timeout: u8,
    pub hcaptcha: CaptchaItemConfig,
    pub turnstile: CaptchaItemConfig,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: String,
    pub web: WebConfig,
    pub db: DbConfig,
    pub session: SessionConfig,
    pub mails: Vec<MailConfig>,
    pub protected_content: ProtectedContentConfig,
    pub captcha: CaptchaConfig,
}

impl Config {
    pub fn from_toml() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?
            .try_deserialize()
    }
}
