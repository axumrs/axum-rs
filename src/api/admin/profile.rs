use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    api::{get_pool, log_error},
    form, mid, model, resp, utils, ArcAppState, Error, Result,
};

pub async fn change_pwd(
    State(state): State<ArcAppState>,
    mid::AdminAuth { admin, .. }: mid::AdminAuth,
    Json(frm): Json<form::profile::ChangePassword>,
) -> Result<resp::JsonAffResp> {
    let handler_name = "admin/profile/change_pwd";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if frm.password == frm.new_password {
        return Err(Error::new("玩呢？一样的密码改个寂寞"));
    }
    if frm.new_password != frm.re_password {
        return Err(Error::new("两次输入的密码不一致"));
    }

    let p = get_pool(&state);

    let m = match model::admin::Admin::find(
        &*p,
        &model::admin::AdminFindFilter {
            by: model::admin::AdminFindBy::Id(admin.id.clone()),
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?
    {
        Some(v) => v,
        None => return Err(Error::new("管理员不存在")),
    };

    if !utils::password::verify(&frm.password, &m.password).map_err(log_error(handler_name))? {
        return Err(Error::new("密码错误"));
    }

    let password = utils::password::hash(&frm.new_password).map_err(log_error(handler_name))?;

    let aff = match sqlx::query(r#"UPDATE "admins" SET "password"=$1 WHERE "id"=$2"#)
        .bind(&password)
        .bind(&admin.id)
        .execute(&*p)
        .await
    {
        Err(e) => return Err(e.into()).map_err(log_error(handler_name)),
        Ok(v) => v.rows_affected(),
    };

    Ok(resp::ok(resp::AffResp { aff }))
}
