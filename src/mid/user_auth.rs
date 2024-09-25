use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{model, ArcAppState, Error};

pub struct UserAuth(Option<model::user::User>);

impl UserAuth {
    pub fn user(&self) -> &Option<model::user::User> {
        &self.0
    }
}

#[async_trait]
impl FromRequestParts<ArcAppState> for UserAuth {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &ArcAppState,
    ) -> Result<Self, Self::Rejection> {
        let u = super::auth_fn::get_auth_user(&parts.headers).await?;

        Ok(UserAuth(u))
    }
}
