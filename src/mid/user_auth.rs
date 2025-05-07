use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::Local;

use crate::{model, utils, ArcAppState, Error};

pub struct UserAuth {
    pub user: Option<model::user::User>,
    pub token: Option<String>,
}

impl UserAuth {
    pub fn user_opt(&self) -> &Option<model::user::User> {
        &self.user
    }
    pub fn user(&self) -> crate::Result<&model::user::User> {
        match self.user_opt() {
            Some(v) => Ok(v),
            None => Err(Error::new("UNAUTHORIZED-请登录")),
        }
    }

    pub fn token_opt(&self) -> &Option<String> {
        &self.token
    }

    pub fn token(&self) -> crate::Result<&str> {
        match self.token_opt() {
            Some(v) => Ok(v),
            None => Err(Error::new("UNAUTHORIZED-请登录")),
        }
    }
}

impl FromRequestParts<ArcAppState> for UserAuth {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcAppState,
    ) -> Result<Self, Self::Rejection> {
        let token = match utils::http::get_auth_token(&parts.headers) {
            Some(v) => v,
            None => {
                return Ok(UserAuth {
                    user: None,
                    token: None,
                })
            }
        };

        let sesc = match model::session::Session::find(
            &*state.pool,
            &model::session::SessionFindFilter {
                token: Some(token.into()),
                user_id: None,
                is_admin: Some(false),
                id: None,
            },
        )
        .await?
        {
            Some(v) => v,
            None => return Err(Error::new("UNAUTHORIZED-非法令牌")),
        };

        if sesc.expire_time < Local::now() {
            return Err(Error::new("UNAUTHORIZED-登录已过期"));
        }

        let u = match model::user::User::find(
            &*state.pool,
            &model::user::UserFindFilter {
                by: model::user::UserFindBy::Id(sesc.user_id),
                status: Some(model::user::Status::Actived),
            },
        )
        .await?
        {
            Some(v) => v,
            None => return Err(Error::new("UNAUTHORIZED-不存在的用户")),
        };

        Ok(UserAuth {
            user: Some(u),
            token: Some(token.into()),
        })
    }
}
