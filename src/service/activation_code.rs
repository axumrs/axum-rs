use chrono::Local;
use sqlx::{PgExecutor, PgPool};

use crate::{model, utils, Error, Result};

pub async fn get<'a>(
    c: impl PgExecutor<'a>,
    email: &str,
    kind: model::activation_code::Kind,
    code: Option<String>,
) -> sqlx::Result<Option<model::activation_code::ActivationCode>> {
    let expired = Local::now();
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
        if (expired - m.dateline).num_minutes() < 5 {
            return Ok(Some(m));
        }
    }
    Ok(None)
}

pub async fn add(
    p: &PgPool,
    m: model::activation_code::ActivationCode,
) -> Result<model::activation_code::ActivationCode> {
    let mut tx = p.begin().await.map_err(Error::from)?;

    let exists = match get(&mut *tx, &m.email, m.kind.clone(), None).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if exists.is_some() {
        return Err(Error::new("请求过于频繁，请稍后再试"));
    }

    let id = utils::id::new();
    let m = model::activation_code::ActivationCode { id, ..m };

    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(m)
}
