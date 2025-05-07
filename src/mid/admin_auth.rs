use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::Local;

use crate::{model, utils, ArcAppState, Error};

pub struct AdminAuth {
    pub admin: model::admin::Admin,
    pub token: String,
}

impl AdminAuth {
    pub fn admin(&self) -> &model::admin::Admin {
        &self.admin
    }
}

impl FromRequestParts<ArcAppState> for AdminAuth {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcAppState,
    ) -> Result<Self, Self::Rejection> {
        let token = match utils::http::get_auth_token(&parts.headers) {
            Some(v) => v,
            None => return Err(Error::new("未授权")),
        };

        let sesc = match model::session::Session::find(
            &*state.pool,
            &model::session::SessionFindFilter {
                token: Some(token.into()),
                user_id: None,
                is_admin: Some(true),
                id: None,
            },
        )
        .await?
        {
            Some(v) => v,
            None => return Err(Error::new("非法令牌")),
        };

        if sesc.expire_time < Local::now() {
            return Err(Error::new("登录已过期"));
        }

        let u = match model::admin::Admin::find(
            &*state.pool,
            &model::admin::AdminFindFilter {
                by: model::admin::AdminFindBy::Id(sesc.user_id),
            },
        )
        .await?
        {
            Some(v) => v,
            None => return Err(Error::new("不存在的用户")),
        };
        Ok(AdminAuth {
            admin: u,
            token: token.into(),
        })
    }
}
