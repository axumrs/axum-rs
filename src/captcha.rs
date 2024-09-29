use serde::Deserialize;

use crate::{config, Error, Result};

pub struct Captcha<'a> {
    pub kind: config::CaptchaKind,
    pub secret: &'a str,
    pub validation_url: &'a str,
    pub timeout: u8,
}

#[derive(Deserialize)]
pub struct Response {
    pub success: bool,
}

impl<'a> Captcha<'a> {
    pub fn from_cfg(kind: config::CaptchaKind, cfg: &'a config::Config) -> Self {
        let (secret, validation_url) = match kind {
            config::CaptchaKind::HCaptcha => (
                &cfg.captcha.hcaptcha.secret_key,
                &cfg.captcha.hcaptcha.validation_url,
            ),
            config::CaptchaKind::Turnstile => (
                &cfg.captcha.turnstile.secret_key,
                &cfg.captcha.turnstile.validation_url,
            ),
        };
        Self {
            kind,
            secret,
            validation_url,
            timeout: cfg.captcha.timeout,
        }
    }
    pub fn hcaptch(cfg: &'a config::Config) -> Self {
        Self::from_cfg(config::CaptchaKind::HCaptcha, cfg)
    }

    pub fn turnstile(cfg: &'a config::Config) -> Self {
        Self::from_cfg(config::CaptchaKind::Turnstile, cfg)
    }
}

pub async fn verify<'a>(c: Captcha<'a>, token: &'a str) -> Result<Response> {
    let form = [("secret", c.secret), ("response", token)];
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(c.timeout as u64))
        .build()
        .map_err(Error::from)?;
    let res = client.post(c.validation_url).form(&form).send().await?;
    let res = res.json().await?;
    Ok(res)
}
