use std::sync::Arc;

use axum::Extension;

use crate::{
    db::subject,
    handler_helper::{get_conn, log_error},
    model::{State, Subject},
    IDResponse, JsonRespone, Response, Result,
};

pub async fn add(Extension(state): Extension<Arc<State>>) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/subject/add";

    let conn = get_conn(&state);
    let id = subject::add(
        &conn,
        &Subject {
            name: "你好，世界".to_string(),
            slug: "hello-world-1".to_string(),
            summary: "Hello, 世界".to_string(),
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;
    tracing::debug!("new subject id: {}", id);
    Ok(Response::ok(IDResponse { id }).to_json())
}
