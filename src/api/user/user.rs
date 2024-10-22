use axum::extract::State;

use crate::{
    api::{get_pool, log_error},
    mid, resp, ArcAppState, Error, Result,
};

pub async fn logout(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
) -> Result<resp::JsonAffResp> {
    let handler_name = "user/logout";

    let user = user_auth.user().map_err(log_error(handler_name))?;
    let token = user_auth.token().map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let aff = sqlx::query("DELETE FROM sessions WHERE token=$1 AND is_admin=false AND user_id=$2")
        .bind(token)
        .bind(&user.id)
        .execute(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?
        .rows_affected();

    Ok(resp::ok(resp::AffResp { aff }))
}
