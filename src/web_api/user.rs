use std::sync::Arc;

use axum::{extract::Query, Extension, Json};
use validator::Validate;

use crate::{
    db::{user, user_login_log, Paginate},
    form::user as form,
    handler_helper::{get_conn, log_error},
    jwt,
    middleware::UserAuth,
    model::{self, State},
    rdb, Error, IDResponse, JsonRespone, Response, Result,
};

pub async fn online_derive(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<Vec<jwt::UserClaimsData>>> {
    let handler_name = "web/user/online_derive";

    let list = rdb::user::get_online_list(&state.rds, &state.cfg, &cd.email)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(list).to_json())
}

pub async fn login_log(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
    Query(frm): Query<crate::form::PaginateForm>,
) -> Result<JsonRespone<Paginate<model::UserLoginLogFull>>> {
    let handler_name = "web/user/login_log";

    let conn = get_conn(&state);
    let p = user_login_log::list(
        &conn,
        &model::PaginateWith {
            page: frm.page,
            page_size: frm.page_size,
        },
        cd.id,
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}

pub async fn logout(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<jwt::UserClaimsData>> {
    let handler_name = "web/user/logout";
    rdb::user::del_online(&state.rds, &state.cfg, &cd.email, &cd.online_id)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Response::ok(cd).to_json())
}

pub async fn basic_info(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<model::UserBasicInfo>> {
    let handler_name = "web/user/basic_info";

    let conn = get_conn(&state);
    let u = user::basic_info(&conn, cd.id)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(u).to_json())
}

pub async fn check_in(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<model::UserBasicInfo>> {
    let handler_name = "web/user/check_in";

    let conn = get_conn(&state);

    user::check_in(&conn, cd.id, 5)
        .await
        .map_err(log_error(handler_name))?;

    let u = user::basic_info(&conn, cd.id)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(u).to_json())
}

pub async fn change_pwd(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
    Json(frm): Json<form::ChangePassword>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "web/user/change_pwd";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let conn = get_conn(&state);

    user::change_pwd(&conn, &frm.password, &frm.new_password, cd.id)
        .await
        .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id: cd.id }).to_json())
}

pub async fn profile(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
) -> Result<JsonRespone<model::User2Profile>> {
    let handler_name = "web/user/profile";

    let conn = get_conn(&state);

    let up = user::profile(&conn, cd.id)
        .await
        .map_err(log_error(handler_name))?;

    match up {
        Some(up) => Ok(Response::ok(up).to_json()),
        None => Err(Error::not_found("不存在的用户")),
    }
}

pub async fn update_profile(
    Extension(state): Extension<Arc<State>>,
    UserAuth(cd): UserAuth,
    Json(frm): Json<form::Profile>,
) -> Result<JsonRespone<IDResponse>> {
    let handler_name = "web/user/update_profile";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let conn = get_conn(&state);

    let allow_device_num: u8 = match &cd.types {
        &model::UserTypes::Normal => 1,
        &model::UserTypes::Subscriber => {
            if frm.allow_device_num > 3 {
                3
            } else {
                frm.allow_device_num
            }
        }
    };

    let jwt_exp: u8 = match &cd.types {
        &model::UserTypes::Normal => 0,
        &model::UserTypes::Subscriber => {
            if frm.jwt_exp > 120 {
                120
            } else {
                frm.jwt_exp
            }
        }
    };

    user::update_profile(
        &conn,
        &model::User2Profile {
            id: cd.id,
            email: frm.email,
            nickname: frm.nickname,
            password: frm.password,
            allow_device_num,
            jwt_exp,
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(IDResponse { id: cd.id }).to_json())
}
