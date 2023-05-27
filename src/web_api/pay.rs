use std::sync::Arc;

use axum::{extract::Path, Extension};

use crate::{
    db::pay,
    handler_helper::{get_conn, log_error},
    model::{self, State},
    Error, JsonRespone, Response, Result,
};

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u64>,
) -> Result<JsonRespone<model::Pay>> {
    let handler_name = "web/pay/find";
    let conn = get_conn(&state);
    let p = pay::find(&conn, id)
        .await
        .map_err(log_error(handler_name))?;

    match p {
        Some(p) => Ok(Response::ok(p).to_json()),
        None => Err(Error::not_found("不存在的支付信息")),
    }
}
