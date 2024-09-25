use chrono::Local;
use serde::Deserialize;
use validator::Validate;

use crate::{config, model, utils, Result};

#[derive(Deserialize, Validate)]
pub struct AddForm {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 30))]
    pub nickname: String,

    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 6))]
    pub re_password: String,
}

impl AddForm {
    pub fn to_model(self, cfg: &config::SessionConfig) -> Result<model::user::User> {
        let password = utils::password::hash(&self.password)?;
        let u = model::user::User {
            email: self.email,
            nickname: self.nickname,
            password,
            status: model::user::Status::Pending,
            dateline: Local::now(),
            kind: model::user::Kind::Normal,
            session_exp: cfg.default_timeout as i16,
            ..Default::default()
        };

        Ok(u)
    }
}
