use std::sync::Arc;

use axum::{extract::Query, Extension};

use crate::{
    db::{user_read_history, Paginate},
    handler_helper::{get_conn, log_error},
    middleware::UserAuth,
    model::{self, State},
    Error, JsonRespone, Response, Result,
};

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    UserAuth(claims): UserAuth,
    Query(frm): Query<crate::form::PaginateForm>,
) -> Result<JsonRespone<Paginate<model::UserReadHistoryListView>>> {
    let handler_name = "web/user_read_history/list";

    let page = match &claims.types {
        &model::UserTypes::Normal => {
            if frm.page == 0 {
                frm.page
            } else {
                return Err(Error::must_subscribe("普通用户只能查看最新的一页记录"));
            }
        }
        &model::UserTypes::Subscriber => frm.page,
    };

    let with = model::UserReadHistoryListWith {
        user_id: Some(claims.id),
        pw: model::PaginateWith {
            page,
            page_size: frm.page_size,
        },
    };

    let conn = get_conn(&state);

    let p = user_read_history::list(&conn, &with)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}
