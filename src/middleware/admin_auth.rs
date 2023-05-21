use std::sync::Arc;

use axum::{http::Request, middleware::Next, response::Response, TypedHeader};
use headers::{authorization::Bearer, Authorization};

use crate::{jwt, model::State, Result};

pub async fn admin_auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let state = req.extensions().get::<Arc<State>>().unwrap();
    let cfg = &state.cfg.admin_jwt;
    let token = auth.token();
    tracing::debug!("token: {}", token);

    let claims = jwt::token::decode::<jwt::AdminClaimsData>(token, cfg)?;
    tracing::debug!("admin: {:?}", claims);
    Ok(next.run(req).await)
}
