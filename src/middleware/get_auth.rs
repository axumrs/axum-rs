use std::sync::Arc;

use axum::http::header;
use headers::HeaderMap;

use crate::{jwt, model::State, Result};

pub async fn token_from_header(h: &HeaderMap) -> Option<String> {
    let auth_header = h
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    if auth_header.is_none() {
        return None;
    }

    let auth_header_arr: Vec<&str> = auth_header.unwrap_or_default().split(" ").collect();
    // tracing::debug!("{:?}", auth_header_arr);

    if auth_header_arr.len() != 2 {
        return None;
    }

    Some(auth_header_arr[1].to_string())
}

pub async fn claims_from_header(
    h: &HeaderMap,
    state: &Arc<State>,
) -> Result<Option<jwt::Claims<jwt::UserClaimsData>>> {
    let token = token_from_header(h).await;
    if token.is_none() {
        return Ok(None);
    }
    let token = token.unwrap();

    // let state = parts.extensions.get::<Arc<State>>().unwrap();
    // tracing::debug!("{}", &state.cfg.user_jwt.secret_key);
    match jwt::token::decode(&token, &state.cfg.user_jwt) {
        Ok(cd) => Ok(Some(cd)),
        Err(_) => Ok(None),
    }
}
