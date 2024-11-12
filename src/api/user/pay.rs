use axum::extract::State;

use crate::{mid, resp, ArcAppState, Result};

/// 支付
pub async fn add(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
) -> Result<resp::JsonIDResp> {
    unimplemented!()
}

/// 完成支付
pub async fn complete(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
) -> Result<resp::JsonIDResp> {
    unimplemented!()
}

/// 支付详情
pub async fn detail(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
) -> Result<resp::JsonIDResp> {
    unimplemented!()
}
