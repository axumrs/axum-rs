use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use validator::Validate;

use crate::{
    captcha::Captcha,
    db::{topic, Paginate},
    form::topic as form,
    handler_helper::{get_conn, log_error},
    middleware::{PurchaseSubject, UserInfoOption},
    model::{self, State},
    protected_content, rdb, Error, JsonRespone, Response, Result,
};

pub async fn top10(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonRespone<Vec<model::Topic2WebList>>> {
    let handler_name = "web/topic/top10";

    let conn = get_conn(&state);
    let p = topic::list2web(
        &conn,
        &model::Topic2WebListWith {
            order_by_hit: true,
            page: 0,
            page_size: 10,
            ..Default::default()
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p.data).to_json())
}
pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(frm): Query<form::List2Web>,
) -> Result<JsonRespone<Paginate<model::Topic2WebList>>> {
    let handler_name = "web/topic/list";

    let conn = get_conn(&state);
    let p = topic::list2web(
        &conn,
        &model::Topic2WebListWith {
            subject_name: frm.subject_name,
            subject_slug: frm.subject_slug,
            page: frm.page,
            page_size: frm.page_size,
            order_by_hit: frm.order_by_hit.unwrap_or(false),
            title: frm.title,
            tag_name: frm.tag_name,
            asc_order: frm.asc_order.unwrap_or(false),
        },
    )
    .await
    .map_err(log_error(handler_name))?;

    Ok(Response::ok(p).to_json())
}

pub async fn detail(
    Extension(state): Extension<Arc<State>>,
    Path(frm): Path<form::Detail>,
    PurchaseSubject(purchased_subject): PurchaseSubject,
    UserInfoOption(claims): UserInfoOption,
) -> Result<JsonRespone<model::ProtectedTopic2WebDetail>> {
    let handler_name = "web/topic/detail";

    let conn = get_conn(&state);
    let t = topic::detail2web(&conn, &frm.slug, &frm.subject_slug)
        .await
        .map_err(log_error(handler_name))?;
    if let Some(t) = t {
        if t.price > 0 && (!(purchased_subject.is_some() || t.try_readable)) {
            return Ok(Response::err_with_data(
                &Error::unpurchased(),
                model::ProtectedTopic2WebDetail::with_out(model::Topic2WebDetail {
                    html: "你需要购买".to_string(),
                    ..t
                }),
            )
            .to_json());
        }
        let user_type = if let Some(claims) = claims {
            Some(claims.data.types)
        } else {
            None
        };

        let pc = protected_content(&t.html, &state.cfg, &user_type, &state.rds)
            .await
            .map_err(log_error(handler_name))?;

        let out = if let Some((pc_html, pc_ids)) = pc {
            tracing::debug!("pc ids: {:?}", pc_ids);
            let captcha_type = match &user_type {
                Some(_) => (&state.cfg.protected_topic).normal_user_captcha.clone(),
                None => (&state.cfg.protected_topic).guest_captcha.clone(),
            };
            model::ProtectedTopic2WebDetail {
                detail: model::Topic2WebDetail { html: pc_html, ..t },
                captcha_type,
                protect_ids: pc_ids,
            }
        } else {
            // 无需内容保护
            model::ProtectedTopic2WebDetail::with_out(t)
        };

        return Ok(Response::ok(out).to_json());
    }

    return Err(Error::not_found("不存在的文章"));
}

pub async fn get_protected_content(
    Extension(state): Extension<Arc<State>>,
    Json(frm): Json<crate::form::protected_content::Get>,
) -> Result<JsonRespone<Vec<protected_content::ProtectedContent>>> {
    let handler_name = "web/topic/get_protected_content";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if !Captcha::new_hcaptcha(&state.cfg.hcaptcha.secret_key)
        .verify(&frm.response)
        .await
        .map_err(log_error(handler_name))?
    {
        return Err(Error::captcha_failed());
    }

    let mut list = Vec::with_capacity(frm.protect_ids.len());
    for id in frm.protect_ids.iter() {
        let key = rdb::protected_topic_keyname(&state.cfg, id);
        let pc = rdb::get(&state.rds, &key)
            .await
            .map_err(log_error(handler_name))?;
        if let Some(pc) = pc {
            list.push(protected_content::ProtectedContent {
                id: id.to_string(),
                content: pc,
            });
        }
    }

    Ok(Response::ok(list).to_json())
}
