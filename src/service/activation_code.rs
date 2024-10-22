use chrono::{Duration, Local};
use sqlx::{PgExecutor, PgPool};

use crate::{model, utils, Error, Result};

pub async fn get<'a>(
    c: impl PgExecutor<'a>,
    email: &str,
    kind: model::activation_code::Kind,
    code: Option<String>,
) -> sqlx::Result<Option<model::activation_code::ActivationCode>> {
    let expired = Local::now() + Duration::minutes(5);
    let m = model::activation_code::ActivationCode::find(
        c,
        &model::activation_code::ActivationCodeFindFilter {
            id: None,
            email: Some(email.to_string()),
            kind: Some(kind),
            code,
        },
    )
    .await?;

    if let Some(m) = m {
        if expired >= m.expire_time {
            return Ok(Some(m));
        }
    }
    Ok(None)
}

pub async fn exists<'a>(
    c: impl PgExecutor<'a>,
    email: &str,
    kind: &model::activation_code::Kind,
) -> sqlx::Result<bool> {
    let count: (i64,) = sqlx::query_as(
        "SELECT count(*) FROM activation_codes WHERE email=$1 AND kind=$2 AND expire_time>=$3",
    )
    .bind(email)
    .bind(kind)
    .bind(Local::now())
    .fetch_one(c)
    .await?;
    Ok(count.0 > 0)
}

pub async fn add(
    p: &PgPool,
    m: model::activation_code::ActivationCode,
) -> Result<model::activation_code::ActivationCode> {
    let mut tx = p.begin().await.map_err(Error::from)?;

    let exists = match exists(&mut *tx, &m.email, &m.kind).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if exists {
        return Err(Error::new("请求过于频繁，请稍后再试"));
    }

    let id = utils::id::new();
    let expire_time = m.dateline + Duration::minutes(5);
    let m = model::activation_code::ActivationCode {
        id,
        expire_time,
        ..m
    };

    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(m)
}
