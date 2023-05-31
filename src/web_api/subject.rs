use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};

use crate::{
    db::{subject, user_purchased_subject, Paginate},
    form::subject as form,
    handler_helper::{get_conn, log_error},
    middleware::{PurchaseSubject, UserInfoOption},
    model::{self, State},
    Error, JsonRespone, Response, Result,
};

pub async fn top4(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonRespone<Vec<model::Subject>>> {
    let handler_name = "web/subject/top4";

    let conn = get_conn(&state);
    let p = subject::list(
        &conn,
        model::SubjectListWith {
            is_del: Some(false),
            page: 0,
            page_size: 4,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p.data).to_json())
}

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<form::List>,
    UserInfoOption(user_info): UserInfoOption,
) -> Result<JsonRespone<Paginate<model::SubjectIfPurchased>>> {
    let handler_name = "web/subject/list";

    // tracing::debug!("{:?}", user_info);

    let conn = get_conn(&state);

    let p = subject::list(
        &conn,
        model::SubjectListWith {
            page: frm.page,
            page_size: frm.page_size,
            is_del: Some(false),
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    let mut subject_ids = Vec::with_capacity(p.data.len());
    for s in p.data.iter() {
        subject_ids.push(s.id);
    }

    let user_purchased_subject_list = match user_info {
        Some(claims) => {
            // tracing::debug!("claims: {:?}", claims);
            user_purchased_subject::select_in(&conn, &subject_ids, claims.data.id, Some(false))
                .await
                .map_err(log_error(handler_name))?
        }
        None => vec![],
    };
    // tracing::debug!(
    //     "user_purchased_subject_list {:?}",
    //     user_purchased_subject_list
    // );
    let mut subject_if_purchased_list: Vec<model::SubjectIfPurchased> =
        Vec::with_capacity(p.data.len());
    for s in p.data.iter() {
        let ups = user_purchased_subject_list.iter().find(|&i| i.id == s.id);
        //tracing::debug!("subject {:?}, ups {:?}", s, ups);
        if let Some(ups_item) = ups {
            subject_if_purchased_list.push(ups_item.into());
        } else {
            subject_if_purchased_list.push(s.into());
        }
    }

    Ok(Response::ok(Paginate::new(
        p.total,
        p.page,
        p.page_size,
        subject_if_purchased_list,
    ))
    .to_json())
}

pub async fn detail(
    Extension(state): Extension<Arc<State>>,
    Path(slug): Path<String>,
    PurchaseSubject(purchased_subject): PurchaseSubject,
) -> Result<JsonRespone<model::SubjectIfPurchased>> {
    let handler_name = "web/subject/detail";

    if let Some(purchased_subject) = purchased_subject {
        return Ok(Response::ok(purchased_subject.into()).to_json());
    }

    let conn = get_conn(&state);
    let s = subject::find(&conn, model::SubjectFindBy::Slug(&slug), Some(false))
        .await
        .map_err(log_error(handler_name))?;

    match s {
        Some(s) => Ok(Response::ok(s.into()).to_json()),
        None => Err(Error::not_found("不存在的专题")),
    }
}
