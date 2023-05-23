use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};

use crate::{
    db::{user, user_login_log, Paginate},
    form::user as form,
    handler_helper::{get_conn, log_error},
    jwt,
    model::{self, State},
    rdb, Error, IDResponse, JsonRespone, Response, Result,
};

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<form::List>,
) -> Result<JsonRespone<Paginate<model::User>>> {
    let handler_name = "admin/user/list";

    let conn = get_conn(&state);
    let p = user::list(
        &conn,
        &model::UserListWith {
            email: frm.email,
            nickname: frm.nickname,
            status: frm.status,
            types: frm.types,
            is_del: frm.is_del,
            page: frm.page,
            page_size: frm.page_size,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}

pub async fn add(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::Create>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/user/add";

    let conn = get_conn(&state);
    let id = user::add(
        &conn,
        &model::User {
            email: frm.email,
            nickname: frm.nickname,
            password: frm.password,
            dateline: chrono::Local::now(),
            status: frm.status.unwrap_or(model::UserStatus::Actived),
            types: frm.types.unwrap_or_default(),
            sub_exp: frm.sub_exp,
            points: frm.points,
            allow_device_num: frm.allow_device_num,
            jwt_exp: frm.jwt_exp,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<model::User>> {
    let handler_name = "admin/user/find";

    let conn = get_conn(&state);
    let u = user::find(&*conn, &model::UserFindBy::ID(id), None)
        .await
        .map_err(log_error(handler_name))?;

    match u {
        Some(u) => Ok(Response::ok(u).to_json()),
        None => Err(Error::not_found("不存在的用户")),
    }
}

pub async fn edit(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<form::Update>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/user/edit";

    let conn = get_conn(&state);
    user::edit(
        &conn,
        &model::UserEdit2Admin {
            id: frm.id,
            email: frm.email,
            nickname: frm.nickname,
            password: frm.password,
            status: frm.status,
            types: frm.types,
            sub_exp: frm.sub_exp,
            points: frm.points,
            allow_device_num: frm.allow_device_num,
            jwt_exp: frm.jwt_exp,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id: frm.id }).to_json())
}

pub async fn del(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/user/del";

    let conn = get_conn(&state);
    user::del_or_restore(&conn, id, true)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}
pub async fn restore(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/user/restore";

    let conn = get_conn(&state);
    user::del_or_restore(&conn, id, false)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn active(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/user/active";

    let conn = get_conn(&state);
    user::change_status(&conn, id, &model::UserStatus::Actived)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn freeze(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/user/freeze";

    let conn = get_conn(&state);
    user::change_status(&conn, id, &model::UserStatus::Freezed)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn pending(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<u32>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "admin/user/pending";

    let conn = get_conn(&state);
    user::change_status(&conn, id, &model::UserStatus::Pending)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id }).to_json())
}

pub async fn online_drive(
    Extension(state): Extension<Arc<State>>,
    Path(email): Path<String>,
) -> Result<JsonRespone<Vec<jwt::UserClaimsData>>> {
    let handler_name = "admin/user/online_drive";

    let list = rdb::user::get_online_list(&state.rds, &state.cfg, &email)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(list).to_json())
}

pub async fn login_log(
    Extension(state): Extension<Arc<State>>,
    Path(user_id): Path<u32>,
    Query(frm): Query<crate::form::PaginateForm>,
) -> Result<JsonRespone<Paginate<model::UserLoginLogFull>>> {
    let handler_name = "admin/user/login_log";

    let conn = get_conn(&state);
    let p = user_login_log::list(
        &conn,
        &model::PaginateWith {
            page: frm.page,
            page_size: frm.page_size,
        },
        user_id,
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}
