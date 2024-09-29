use std::sync::Arc;

use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, response::Response},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::{config, Error, Result};

pub struct Data {
    pub subject: String, // 邮件主题
    pub body: String,    // 邮件内容
    pub to: String,      // 收件人
}

impl Data {
    pub fn to_message(&self, cfg: &config::MailConfig) -> Result<Message> {
        let user = cfg.user.as_str().parse().map_err(Error::from)?;
        let to = self.to.parse().map_err(Error::from)?;
        Message::builder()
            .from(user)
            .to(to)
            .subject(self.subject.as_str())
            .header(ContentType::TEXT_PLAIN)
            .body(self.body.clone())
            .map_err(Error::from)
    }
}

pub async fn send(cfg: Arc<config::Config>, m: Data) -> Result<Response> {
    let cfg = cfg.get_mail()?;
    let message = m.to_message(cfg)?;
    let creds = Credentials::new(cfg.user.clone(), cfg.password.clone());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.smtp)
        .map_err(Error::from)?
        .credentials(creds)
        .build();
    mailer.send(message).await.map_err(Error::from)
}
