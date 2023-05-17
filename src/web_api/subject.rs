use std::sync::Arc;

use axum::Extension;

use crate::{
    db::subject,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    JsonRespone, Response, Result,
};

pub async fn top4(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonRespone<Vec<model::Subject>>> {
    let handler_name = "web/subject/top4";

    let conn = get_conn(&state);
    let p = subject::list(
        &conn,
        model::SubjectListWith {
            is_del: Some(false),
            page: 0,
            page_size: 4,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p.data).to_json())
}
