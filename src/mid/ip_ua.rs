use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

use crate::{utils, Error};

#[derive(Serialize, Deserialize)]
pub struct IpAndUserAgent {
    pub ip: String,
    pub ip_location: String,
    pub user_agent: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for IpAndUserAgent
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ip = utils::http::get_ip(&parts.headers);
        let user_agent = utils::http::get_user_agent(&parts.headers);
        let ip_location = utils::http::get_cf_location(&parts.headers);

        Ok(Self {
            ip: ip.to_string(),
            user_agent: user_agent.to_string(),
            ip_location: ip_location.to_string(),
        })
    }
}
