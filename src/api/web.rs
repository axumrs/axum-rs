use axum::extract::State;

use crate::{mid::IpAndUserAgent, ArcAppState};

pub async fn ping(State(state): State<ArcAppState>, ip_and_user_agent: IpAndUserAgent) -> String {
    format!(
        "[PONG] prefix: {}, client: {:?}",
        &state.cfg.web.prefix, &ip_and_user_agent
    )
}
