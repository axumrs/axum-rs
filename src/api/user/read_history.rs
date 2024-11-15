use axum::extract::{Query, State};

use crate::{
    api::{get_pool, log_error},
    form, mid, model, resp, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
    Query(frm): Query<form::PageQueryStr>,
) -> Result<resp::JsonResp<model::read_history::ReadHistoriePaginate>> {
    let handler_name = "user/read_history/list";
    let p = get_pool(&state);
    let user = user_auth.user().map_err(log_error(handler_name))?;

    let (page, page_size) = match &user.kind {
        &model::user::Kind::Normal => (0, 5),
        &model::user::Kind::Subscriber | &model::user::Kind::YearlySubscriber => {
            (frm.page(), frm.page_size())
        }
    };

    let data = model::read_history::ReadHistorie::list(
        &*p,
        &model::read_history::ReadHistorieListFilter {
            pq: model::read_history::ReadHistoriePaginateReq { page, page_size },
            order: None,
            user_id: Some(user.id.clone()),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}
