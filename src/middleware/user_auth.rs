use std::sync::Arc;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts},
};

use crate::{jwt, model::State, Error};

pub struct UserAuth(pub jwt::UserClaimsData);

#[async_trait]
impl<S> FromRequestParts<S> for UserAuth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        tracing::debug!("user_auth middleware");

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        // tracing::debug!("{:?}", auth_header);

        if auth_header.is_none() {
            return Err(Error::no_token());
        }

        let auth_header_arr: Vec<&str> = auth_header.unwrap_or_default().split(" ").collect();
        // tracing::debug!("{:?}", auth_header_arr);

        if auth_header_arr.len() != 2 {
            return Err(Error::no_token());
        }

        let auth_str = auth_header_arr[1];
        // tracing::debug!("{}", auth_str);

        let state = parts.extensions.get::<Arc<State>>().unwrap();
        // tracing::debug!("{}", &state.cfg.user_jwt.secret_key);
        let cd = jwt::token::decode(auth_str, &state.cfg.user_jwt)?;

        Ok(Self(cd.data))
    }
}
