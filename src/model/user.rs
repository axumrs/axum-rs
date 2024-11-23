use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{interfaces, utils, Result};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone)]
#[sqlx(type_name = "user_status")]
pub enum Status {
    #[default]
    Pending,
    Actived,
    Freezed,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone)]
#[sqlx(type_name = "user_kind")]
pub enum Kind {
    #[default]
    Normal,
    Subscriber,
    YearlySubscriber,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db, Clone)]
#[db(table = users, pk = id)]
pub struct User {
    #[db(find)]
    #[db(skip_update)]
    pub id: String,

    #[db(find)]
    #[db(exists)]
    #[db(list_opt)]
    #[db(list_opt_like)]
    pub email: String,

    #[db(exists)]
    #[db(list_opt)]
    #[db(list_opt_like)]
    pub nickname: String,

    #[serde(skip)]
    pub password: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub status: Status,

    #[db(skip_update)]
    pub dateline: DateTime<Local>,

    #[db(list_opt)]
    pub kind: Kind,
    pub sub_exp: DateTime<Local>,
    pub points: Decimal,
    pub allow_device_num: i16,
    pub session_exp: i16,
}

impl interfaces::AsAuth for User {}

pub struct UserBuilder(User);
impl UserBuilder {
    pub fn new(email: String, nickname: String, password: String) -> Self {
        Self(User {
            email,
            nickname,
            password,
            ..Default::default()
        })
    }

    pub fn id(self, id: String) -> Self {
        Self(User { id, ..self.0 })
    }

    pub fn status(self, status: Status) -> Self {
        Self(User { status, ..self.0 })
    }

    pub fn kind(self, kind: Kind) -> Self {
        Self(User { kind, ..self.0 })
    }

    pub fn sub_exp(self, sub_exp: DateTime<Local>) -> Self {
        Self(User { sub_exp, ..self.0 })
    }

    pub fn dateline(self, dateline: DateTime<Local>) -> Self {
        Self(User { dateline, ..self.0 })
    }
    pub fn dateline_now(self) -> Self {
        self.dateline(Local::now())
    }

    pub fn points(self, points: Decimal) -> Self {
        Self(User { points, ..self.0 })
    }
    pub fn allow_device_num(self, allow_device_num: i16) -> Self {
        Self(User {
            allow_device_num,
            ..self.0
        })
    }
    pub fn session_exp(self, session_exp: i16) -> Self {
        Self(User {
            session_exp,
            ..self.0
        })
    }

    pub fn build(self) -> Result<User> {
        let password = utils::password::hash(&self.0.password)?;
        Ok(User { password, ..self.0 })
    }
}
