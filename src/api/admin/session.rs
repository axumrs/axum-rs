use axum::extract::State;

use crate::{
    api::{get_pool, log_error},
    mid, resp, ArcAppState, Result,
};

pub async fn logout(
    State(state): State<ArcAppState>,
    mid::AdminAuth { token, .. }: mid::AdminAuth,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/profile/logout";

    let p = get_pool(&state);

    let aff = match sqlx::query("DELETE FROM sessions WHERE token = $1 AND is_admin = true")
        .bind(&token)
        .execute(&*p)
        .await
    {
        Err(e) => return Err(e.into()).map_err(log_error(handler_name)),
        Ok(v) => v.rows_affected(),
    };

    Ok(resp::ok(resp::AffResp { aff }))
}
