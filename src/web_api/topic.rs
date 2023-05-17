use std::sync::Arc;

use axum::Extension;

use crate::{
    db::topic,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    JsonRespone, Response, Result,
};

pub async fn top10(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonRespone<Vec<model::Topic2WebList>>> {
    let handler_name = "web/topic/top10";

    let conn = get_conn(&state);
    let p = topic::list2web(
        &conn,
        &model::Topic2WebListWith {
            order_by_hit: true,
            page: 0,
            page_size: 10,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p.data).to_json())
}
