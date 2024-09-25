use sqlx::PgPool;

use crate::{model, utils, Error, Result};

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
