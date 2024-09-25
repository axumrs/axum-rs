use anyhow::anyhow;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{model, ArcAppState, Error};

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
        _state: &ArcAppState,
    ) -> Result<Self, Self::Rejection> {
        let u = match super::auth_fn::get_auth_user(&parts.headers).await? {
            Some(v) => v,
            None => return Err(anyhow!("查无此人").into()),
        };

        Ok(AdminAuth(u))
    }
}
