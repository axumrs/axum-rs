use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Serialize)]
pub struct VerifyRequest<'a> {
    pub secret: &'a str,
    pub response: &'a str,
}
#[derive(Deserialize)]
pub struct VerifyResponse {
    pub success: bool,
}

enum Provider {
    HCaptcha,
    ReCaptcha,
}

impl Provider {
    fn url(&self) -> &str {
        match self {
            &Self::HCaptcha => "https://hcaptcha.com/siteverify",
            &Self::ReCaptcha => "https://www.google.com/recaptcha/api/siteverify",
        }
    }
}

pub struct Captcha<'a> {
    provider: Provider,
    secret: &'a str,
}
impl<'a> Captcha<'a> {
    fn new(provider: Provider, secret: &'a str) -> Self {
        Self { provider, secret }
    }
    pub fn new_hcaptcha(secret: &'a str) -> Self {
        Self::new(Provider::HCaptcha, secret)
    }
    pub fn new_recaptcha(secret: &'a str) -> Self {
        Self::new(Provider::ReCaptcha, secret)
    }
    pub async fn verify(&self, response: &'a str) -> Result<bool> {
        let url = self.provider.url();
        let req = VerifyRequest {
            secret: self.secret,
            response,
        };
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .form(&req)
            .send()
            .await
            .map_err(Error::from)?;
        let res = res.text().await.map_err(Error::from)?;
        let res: VerifyResponse = serde_json::from_str(&res).map_err(Error::from)?;
        Ok(res.success)
    }
}
