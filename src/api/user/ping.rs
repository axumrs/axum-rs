use axum::extract::State;

use crate::{
    mid::{IpAndUserAgent, UserAuth},
    ArcAppState,
};

pub async fn ping(
    State(state): State<ArcAppState>,
    auth: UserAuth,
    ip_and_user_agent: IpAndUserAgent,
) -> String {
    format!(
        "[PONG] prefix: {}, user: {:?}, client: {:?}",
        &state.cfg.web.prefix,
        auth.user(),
        ip_and_user_agent,
    )
}
