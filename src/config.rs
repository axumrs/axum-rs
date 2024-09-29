use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub addr: String,
    pub prefix: String,
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
    pub admin_timeout: u32,
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
pub struct UploadConfig {
    pub max_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: String,
    pub cleaner_max_try: u32,
    pub web: WebConfig,
    pub db: DbConfig,
    pub session: SessionConfig,
    pub mails: Vec<MailConfig>,
    pub protected_content: ProtectedContentConfig,
    pub captcha: CaptchaConfig,
    pub upload: UploadConfig,
}

impl Config {
    pub fn from_toml() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?
            .try_deserialize()
    }

    pub fn get_mail(&self) -> crate::Result<&MailConfig> {
        let idx = rand::thread_rng().gen_range(0..self.mails.len());
        let m = match self.mails.get(idx) {
            Some(m) => m,
            None => return Err(crate::Error::new("msg")),
        };
        Ok(m)
    }
}
