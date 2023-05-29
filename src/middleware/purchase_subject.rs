use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{
    db,
    handler_helper::{get_conn, log_error},
    middleware::get_auth,
    model::{self, State},
    Error,
};

/// 用于判断用户是否购买了专题
pub struct PurchaseSubject(pub Option<model::UserPurchasedSubject>);

#[async_trait]
impl<S> FromRequestParts<S> for PurchaseSubject
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let handler_name = "middleware/purchase_subject";
        // tracing::debug!("{}", handler_name);

        /*
           /slug => 专题详情
           /subject_slug/slug => 文章详情
        */
        let url = parts.uri.path();
        // tracing::debug!("url: {:?}", url);

        let url_parts: Vec<&str> = url.split("/").filter(|p| !p.is_empty()).collect();

        if url_parts.len() < 1 {
            // tracing::debug!("参数个数错误");
            return Ok(Self(None));
        }

        let subject_slug = url_parts[0];

        // tracing::debug!(
        //     "purchase_subject url: {}, url_parts: {:?}, subject_slug: {}",
        //     url,
        //     url_parts,
        //     subject_slug,
        // );
        let state = parts.extensions.get::<Arc<State>>().unwrap();

        let cd = get_auth::claims_from_header(&parts.headers, state)
            .await
            .map_err(log_error(handler_name))?;

        if cd.is_none() {
            // tracing::debug!("purchase_subject 游客");
            return Ok(Self(None));
        }

        let cd = cd.unwrap();
        let user_id = cd.data.id;

        let conn = get_conn(&state);
        let result = db::user_purchased_subject::find_by_subject(&conn, subject_slug, user_id)
            .await
            .map_err(log_error(handler_name))?;

        // tracing::debug!(
        //     "purchase_subject  user id: {}, is_purchased:{:?}",
        //     user_id,
        //     result,
        // );

        Ok(Self(result))
    }
}
