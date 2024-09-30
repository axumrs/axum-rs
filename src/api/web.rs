use axum::extract::State;

use crate::{mid::IpAndUserAgent, resp, ArcAppState};

pub async fn ping(
    State(state): State<ArcAppState>,
    ip_and_user_agent: IpAndUserAgent,
) -> resp::JsonResp<String> {
    resp::ok(format!(
        "[PONG] prefix: {}, client: {:?}",
        &state.cfg.web.prefix, &ip_and_user_agent
    ))
}
