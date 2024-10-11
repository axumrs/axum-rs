use anyhow::anyhow;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use chrono::Local;

use crate::{model, utils, ArcAppState, Error};

pub struct AdminAuth(model::admin::Admin);

impl AdminAuth {
    pub fn admin(&self) -> &model::admin::Admin {
        &self.0
    }
}

#[async_trait]
impl FromRequestParts<ArcAppState> for AdminAuth {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcAppState,
    ) -> Result<Self, Self::Rejection> {
        let token = match utils::http::get_auth_token(&parts.headers) {
            Some(v) => v,
            None => return Err(anyhow!("未授权").into()),
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
            None => return Err(anyhow!("非法令牌").into()),
        };

        if sesc.expire_time < Local::now() {
            return Err(anyhow!("登录已过期").into());
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
            None => return Err(anyhow!("不存在的管理员").into()),
        };
        Ok(AdminAuth(u))
    }
}
