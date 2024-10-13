use axum::extract::{Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, ArcAppState, Error, Result,
};

pub async fn all(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::ListAll>,
) -> Result<resp::JsonResp<Vec<model::tag::Tag>>> {
    let handler_name = "admin/tag/all";
    let p = get_pool(&state);
    let data = model::tag::Tag::list_all(
        &*p,
        &model::tag::TagListAllFilter {
            limit: frm.limit,
            order: Some("id ASC".into()),
            name: None,
            is_del: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(data))
}
