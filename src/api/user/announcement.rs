use axum::extract::{Path, Query, State};

use crate::{
    api::{get_pool, log_error},
    form, model, resp, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::PageQueryStr>,
) -> Result<resp::JsonResp<model::pagination::Paginate<model::announcement::AnnouncementLite>>> {
    let handler_name = "api/user/announcement/list";
    let p = get_pool(&state);

    let mut q = sqlx::QueryBuilder::new("SELECT id,title,dateline FROM announcements ");
    q.push(" ORDER BY id DESC ")
        .push(" LIMIT ")
        .push_bind(frm.page_size_to_bind())
        .push(" OFFSET ")
        .push_bind(frm.offset_to_bind());

    let ls = q
        .build_query_as()
        .fetch_all(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    let c: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM announcements")
        .fetch_one(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let data = model::pagination::Paginate::quick(c, frm.page(), frm.page_size(), ls);
    Ok(resp::ok(data))
}

pub async fn detail(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonResp<model::announcement::Announcement>> {
    let handler_name = "api/user/announcement/detail";
    let p = get_pool(&state);

    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let data = match model::announcement::Announcement::find(
        &mut *tx,
        &model::announcement::AnnouncementFindFilter { id: Some(id) },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("不存在的公告")).map_err(log_error(handler_name)),
        },
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into()).map_err(log_error(handler_name));
        }
    };

    let data = model::announcement::Announcement {
        hit: data.hit + 1,
        ..data
    };

    if let Err(e) = data.update(&mut *tx).await {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into()).map_err(log_error(handler_name));
    }

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}
