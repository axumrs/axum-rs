use axum::extract::{Path, State};

use crate::{
    api::{get_pool, log_error},
    model, resp, service, ArcAppState, Error, Result,
};

pub async fn take(
    State(state): State<ArcAppState>,
) -> Result<resp::JsonResp<model::promotion::Promotion>> {
    let handler_name = "user/promotion/take";
    let p = get_pool(&state);
    let m = service::promotion::random_take(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    let m = match m {
        Some(v) => v,
        None => return Err(Error::new("没有推广数据")),
    };
    Ok(resp::ok(m))
}

pub async fn get(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonResp<model::promotion::Promotion>> {
    let handler_name = "user/promotion/get";
    if id.len() < 20 {
        return Err(Error::new("参数错误"));
    }
    // 兼容老数据
    let id = if id.len() > 20 {
        id[0..20].to_string()
    } else {
        id
    };

    let p = get_pool(&state);
    let m = model::promotion::Promotion::find(
        &*p,
        &model::promotion::PromotionFindFilter { id: Some(id) },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    let m = match m {
        Some(v) => v,
        None => return Err(Error::new("没有推广数据")),
    };
    Ok(resp::ok(m))
}
