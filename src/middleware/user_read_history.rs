use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{
    db,
    handler_helper::{get_conn, log_error},
    middleware::get_auth,
    model::State,
    Error,
};

pub struct UserReadHistory {}

#[async_trait]
impl<S> FromRequestParts<S> for UserReadHistory
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let handler_name = "middleware/user_read_history";
        tracing::debug!("{}", handler_name);

        let url = parts.uri.path();
        let url_parts: Vec<&str> = url.split("/").filter(|p| !p.is_empty()).collect();

        if url_parts.len() != 2 {
            tracing::debug!("参数个数错误");
            return Ok(Self {});
        }

        let subject_slug = url_parts[0];
        let slug = url_parts[1];

        tracing::debug!(
            "user_read_history url: {}, url_parts: {:?}, subject_slug: {}, slug:{}",
            url,
            url_parts,
            subject_slug,
            slug
        );
        let state = parts.extensions.get::<Arc<State>>().unwrap();

        let cd = get_auth::claims_from_header(&parts.headers, state)
            .await
            .map_err(log_error(handler_name))?;

        if cd.is_none() {
            tracing::debug!("user_read_history 游客");
            return Ok(Self {});
        }

        let cd = cd.unwrap();
        let user_id = cd.data.id;

        let conn = get_conn(&state);
        let id = db::user_read_history::add(
            &conn,
            &crate::model::UserReadHistory {
                user_id,
                subject_slug: subject_slug.to_string(),
                slug: slug.to_string(),
                dateline: chrono::Local::now(),
                is_del: false,
                ..Default::default()
            },
        )
        .await
        .map_err(log_error(handler_name))?;

        tracing::debug!("user_read_history  user id: {}, history id:{}", user_id, id);

        Ok(Self {})
    }
}
