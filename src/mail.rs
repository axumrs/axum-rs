use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, response::Response},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::{config, Error, Result};

pub struct Data<'a> {
    pub subject: &'a str, // 邮件主题
    pub body: &'a str,    // 邮件内容
    pub to: &'a str,      // 收件人
}

impl<'a> Data<'a> {
    pub fn new(subject: &'a str, body: &'a str, to: &'a str) -> Self {
        Self { subject, body, to }
    }
    pub fn to_message(&self, cfg: &config::MailConfig) -> Result<Message> {
        let user = cfg.user.as_str().parse().map_err(Error::from)?;
        let to = self.to.parse().map_err(Error::from)?;
        Message::builder()
            .from(user)
            .to(to)
            .subject(self.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(self.body.to_string())
            .map_err(Error::from)
    }
}

pub async fn send<'a>(cfg: &config::MailConfig, m: &Data<'a>) -> Result<Response> {
    let message = m.to_message(cfg)?;
    let creds = Credentials::new(cfg.user.clone(), cfg.password.clone());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.smtp)
        .map_err(Error::from)?
        .credentials(creds)
        .build();
    mailer.send(message).await.map_err(Error::from)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_send_mail() {
        let cfg = crate::config::MailConfig {
            name: "test".to_string(),
            password: "".to_string(),
            smtp: "".to_string(),
            user: "".to_string(),
        };
        let d = super::Data {
            subject: "abc123是你的验证码",
            body: "欢迎注册，abc123是你的验证码",
            to: "yesen@cock.li",
        };
        super::send(&cfg, &d).await.unwrap();
    }
}
