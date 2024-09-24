use anyhow::anyhow;
use sqlx::PgPool;

use crate::{model, utils, Error, Result};

pub async fn add(p: &PgPool, m: model::tag::Tag) -> Result<model::tag::Tag> {
    let id = utils::id::new();
    let m = model::tag::Tag { id, ..m };

    let mut tx = p.begin().await.map_err(Error::from)?;

    let name_exists = match model::tag::Tag::name_is_exists(&mut *tx, &m.name, None).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if name_exists {
        return Err(anyhow!("标签已存在").into());
    }

    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    };

    tx.commit().await.map_err(Error::from)?;

    Ok(m)
}

pub async fn edit(p: &PgPool, m: &model::tag::Tag) -> Result<u64> {
    if m.id.is_empty() {
        return Err(anyhow!("未指定ID").into());
    }

    let mut tx = p.begin().await.map_err(Error::from)?;

    let name_exists =
        match model::tag::Tag::name_is_exists(&mut *tx, &m.name, Some(m.id.clone())).await {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(e.into());
            }
        };

    if name_exists {
        return Err(anyhow!("标签已存在").into());
    }

    let aff = match m.update(&mut *tx).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    tx.commit().await.map_err(Error::from)?;

    Ok(aff)
}

pub async fn find(p: &PgPool, f: &model::tag::TagFindFilter) -> Result<Option<model::tag::Tag>> {
    model::tag::Tag::find(p, f).await.map_err(Error::from)
}

pub async fn find_by_id(p: &PgPool, id: &str) -> Result<Option<model::tag::Tag>> {
    find(
        p,
        &model::tag::TagFindFilter {
            id: Some(id.to_string()),
            name: None,
        },
    )
    .await
}

pub async fn find_by_name(p: &PgPool, name: &str) -> Result<Option<model::tag::Tag>> {
    find(
        p,
        &model::tag::TagFindFilter {
            id: None,
            name: Some(name.to_string()),
        },
    )
    .await
}
