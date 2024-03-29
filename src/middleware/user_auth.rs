use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{jwt, middleware::get_auth, model::State, Error};

pub struct UserAuth(pub jwt::UserClaimsData);

#[async_trait]
impl<S> FromRequestParts<S> for UserAuth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        tracing::debug!("user_auth middleware");

        let cd = get_auth::claims_from_header(
            &parts.headers,
            parts.extensions.get::<Arc<State>>().unwrap(),
        )
        .await?;

        if cd.is_none() {
            return Err(Error::no_token());
        }

        Ok(Self(cd.unwrap().data))
    }
}
