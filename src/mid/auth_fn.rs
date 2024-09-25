use axum::http::HeaderMap;

use crate::{interfaces::AsAuth, utils, Result};

pub(super) async fn get_auth_user<T: AsAuth>(headers: &HeaderMap) -> Result<Option<T>> {
    let _token = match utils::http::get_auth_token(headers) {
        Some(v) => v,
        None => return Ok(None),
    };

    Ok(None)
}
