use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{jwt, middleware::get_auth, model::State, Error};

pub struct UserInfoOption(pub Option<jwt::Claims<jwt::UserClaimsData>>);

#[async_trait]
impl<S> FromRequestParts<S> for UserInfoOption
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // tracing::debug!("user_info_opt middleware");

        let cd = get_auth::claims_from_header(
            &parts.headers,
            parts.extensions.get::<Arc<State>>().unwrap(),
        )
        .await?;

        // tracing::debug!("{:?}", cd);

        Ok(Self(cd))
    }
}
