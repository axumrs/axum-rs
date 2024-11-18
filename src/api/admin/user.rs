use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Local;
use sqlx::QueryBuilder;
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, model, resp, service, utils, ArcAppState, Error, Result,
};

pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::user::ListForAdmin>,
) -> Result<resp::JsonResp<model::user::UserPaginate>> {
    let handler_name = "admin/user/list";
    let p = get_pool(&state);

    let data = model::user::User::list(
        &*p,
        &model::user::UserListFilter {
            pq: model::user::UserPaginateReq {
                page: frm.pq.page(),
                page_size: frm.pq.page_size(),
            },
            order: None,
            email: frm.email,
            nickname: frm.nickname,
            status: frm.status,
            kind: frm.kind,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(data))
}

pub async fn add(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::user::AddForAdmin>,
) -> Result<resp::JsonIDResp> {
    let handler_name = "admin/user/add";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if frm.base.password != frm.base.re_password {
        return Err(Error::new("两次输入的密码不一致"));
    }

    let password = utils::password::hash(&frm.base.password)
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let p = get_pool(&state);

    let m = service::user::add(
        &*p,
        model::user::User {
            sub_exp: frm.sub_exp(),
            email: frm.base.email,
            nickname: frm.base.nickname,
            password,
            status: frm.status,
            dateline: Local::now(),
            kind: frm.kind,
            points: frm.points,
            allow_device_num: frm.allow_device_num,
            session_exp: frm.session_exp,

            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::IDResp { id: m.id }))
}

pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::user::EditForAdmin>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/user/edit";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if frm.password != frm.re_password {
        return Err(Error::new("两次输入的密码不一致"));
    }

    let p = get_pool(&state);

    let user = match model::user::User::find(
        &*p,
        &model::user::UserFindFilter {
            by: model::user::UserFindBy::Id(frm.id.clone()),
            status: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => {
            let password = if let Some(ref v) = frm.password {
                utils::password::hash(v)
                    .map_err(Error::from)
                    .map_err(log_error(handler_name))?
            } else {
                v.password.clone()
            };
            model::user::User {
                sub_exp: frm.sub_exp(),
                email: frm.email,
                nickname: frm.nickname,
                status: frm.status,
                kind: frm.kind,
                points: frm.points,
                allow_device_num: frm.allow_device_num,
                session_exp: frm.session_exp,
                password,
                ..v
            }
        }
        None => return Err(Error::new("不存在的用户")),
    };

    let aff = service::user::edit(&*p, &user)
        .await
        .map_err(log_error(handler_name))?;

    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn del(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/user/del";
    let p = get_pool(&state);
    let aff = model::user::User::real_del(&*p, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(resp::AffResp { aff }))
}

pub async fn search(
    State(state): State<ArcAppState>,
    Query(frm): Query<form::user::SearchForAdmin>,
) -> Result<resp::JsonResp<Vec<model::user::User>>> {
    let handler_name = "admin/user/search";

    let p = get_pool(&state);
    let sql = format!(
        "SELECT {} FROM {:?} WHERE 1=1 ",
        &model::user::User::fields(),
        &model::user::User::table()
    );

    let mut q = QueryBuilder::new(&sql);
    if let Some(ref user_id) = frm.user_id {
        q.push(" AND id =").push_bind(user_id);
    } else {
        let keyword = format!("%{}%", frm.q);
        q.push(" AND (email ILIKE ")
            .push_bind(keyword.clone())
            .push("  OR nickname ILIKE ")
            .push_bind(keyword)
            .push(")");
    }
    // q.push(" AND status=")
    //     .push_bind(&model::user::Status::Actived);
    q.push(" ORDER BY id DESC LIMIT 30");

    tracing::debug!("sql: {}", q.sql());
    let rows = q
        .build_query_as()
        .fetch_all(&*p)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(resp::ok(rows))
}
