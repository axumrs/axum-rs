use axum::extract::State;
use chrono::Local;
use rand::Rng;
use rust_decimal::Decimal;

use crate::{
    api::{get_pool, log_error},
    mid, model, resp,
    utils::{self, dt},
    ArcAppState, Error, Result,
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

pub async fn check_in(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
) -> Result<resp::JsonResp<i16>> {
    let handler_name = "user/check_in";
    let user = user_auth.user().map_err(log_error(handler_name))?;
    let max_point: i16 = match &user.kind {
        &model::user::Kind::Normal => 10,
        &model::user::Kind::Subscriber => 20,
        &model::user::Kind::YearlySubscriber => 30,
    };

    let points = rand::thread_rng().gen_range(1..=max_point);

    let p = get_pool(&state);

    let mut tx = p
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 是否已经签到
    let (start, end) = dt::today();
    let has_check_in_count: (i64,) = match sqlx::query_as(
        "SELECT COUNT(*) FROM check_in_logs WHERE user_id=$1 AND (dateline BETWEEN $2 AND $3)",
    )
    .bind(&user.id)
    .bind(&start)
    .bind(&end)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            return Err(e.into());
        }
    };

    if has_check_in_count.0 > 0 {
        return Err(Error::new("今天已经签过到了"));
    }

    // 签到日志
    let cil = model::check_in_log::CheckInLog {
        id: utils::id::new(),
        user_id: user.id.clone(),
        points,
        dateline: Local::now(),
    };

    if let Err(e) = cil.insert(&mut *tx).await {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into());
    }

    // 更新用户积分
    let points_dec = Decimal::from_i128_with_scale(points as i128, 0);
    if let Err(e) = sqlx::query("UPDATE users SET points=points+$1 WHERE id=$2")
        .bind(&points_dec)
        .bind(&user.id)
        .execute(&mut *tx)
        .await
    {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(e.into());
    }

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(points))
}

pub async fn session_list(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
) -> Result<resp::JsonResp<Vec<model::session::Session>>> {
    let handler_name = "user/session_list";
    let user = user_auth.user().map_err(log_error(handler_name))?;

    let p = get_pool(&state);
    let data = model::session::Session::list_all(
        &*p,
        &model::session::SessionListAllFilter {
            limit: Some(10),
            order: None,
            user_id: Some(user.id.clone()),
            token: None,
            is_admin: Some(false),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(data))
}

pub async fn login_log_list(
    State(state): State<ArcAppState>,
    user_auth: mid::UserAuth,
) -> Result<resp::JsonResp<Vec<model::login_log::LoginLog>>> {
    let handler_name = "user/login_log_list";
    let user = user_auth.user().map_err(log_error(handler_name))?;

    let p = get_pool(&state);
    let data = model::login_log::LoginLog::list_all(
        &*p,
        &model::login_log::LoginLogListAllFilter {
            limit: Some(30),
            order: None,
            user_id: Some(user.id.clone()),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;
    Ok(resp::ok(data))
}
