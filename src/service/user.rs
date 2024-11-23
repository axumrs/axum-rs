use chrono::Local;
use sqlx::PgPool;

use crate::{model, utils, Error, Result};

use super::Tx;

pub async fn add(p: &PgPool, user: model::user::User) -> Result<model::user::User> {
    let id = utils::id::new();
    let user = model::user::User { id, ..user };

    let mut tx = p.begin().await.map_err(Error::from)?;

    let email_exists = match model::user::User::email_is_exists(&mut *tx, &user.email, None).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if email_exists {
        return Err(Error::new("邮箱已存在"));
    }

    let nickname_exists =
        match model::user::User::nickname_is_exists(&mut *tx, &user.nickname, None).await {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(e.into());
            }
        };

    if nickname_exists {
        return Err(Error::new("昵称已存在"));
    }

    if let Err(e) = user.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(user)
}

pub async fn edit(p: &PgPool, user: &model::user::User) -> Result<u64> {
    if user.id.is_empty() {
        return Err(Error::new("未指定ID"));
    }

    let mut tx = p.begin().await.map_err(Error::from)?;

    let email_exists = match model::user::User::email_is_exists(
        &mut *tx,
        &user.email,
        Some(user.id.clone()),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if email_exists {
        return Err(Error::new("邮箱已存在"));
    }

    let nickname_exists = match model::user::User::nickname_is_exists(
        &mut *tx,
        &user.nickname,
        Some(user.id.clone()),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if nickname_exists {
        return Err(Error::new("昵称已存在"));
    }

    let aff = match user.update(&mut *tx).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    tx.commit().await.map_err(Error::from)?;
    Ok(aff)
}

/// 更新订阅时间
pub async fn update_subscribe(
    tx: &mut Tx<'_>,
    user_id: &str,
    duration: i16,
    num: i16,
) -> Result<u64> {
    let duration = duration * num;

    // 是否年付
    let is_yearly = duration >= 365;

    let user = match model::user::User::find(
        &mut **tx,
        &model::user::UserFindFilter {
            by: model::user::UserFindBy::Id(user_id.into()),
            status: Some(model::user::Status::Actived),
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("不存在的用户")),
        },
        Err(e) => return Err(Error::from(e)),
    };

    let user = if duration > 0 {
        // 当前订阅过期？今天开始
        let now = Local::now();
        let start_date = if user.sub_exp <= now {
            now
        } else {
            user.sub_exp
        };
        let sub_exp = start_date + chrono::Duration::days(duration as i64);

        let kind = if is_yearly {
            model::user::Kind::YearlySubscriber
        } else {
            model::user::Kind::Subscriber
        };
        model::user::User {
            sub_exp,
            kind,
            ..user
        }
    } else {
        model::user::User {
            kind: model::user::Kind::Normal,
            ..user
        }
    };
    user.update(&mut **tx).await.map_err(Error::from)?;
    Ok(0)
}
