use anyhow::anyhow;
use sqlx::PgPool;

use crate::{model::subject, utils, Error, Result};

pub async fn add(p: &PgPool, m: subject::Subject) -> Result<subject::Subject> {
    let id = utils::id::new();
    let m = subject::Subject { id, ..m };

    let mut tx = p.begin().await.map_err(Error::from)?;

    let slug_exists = match subject::Subject::slug_is_exists(&mut *tx, &m.slug, None).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if slug_exists {
        return Err(anyhow!("slug已存在").into());
    }

    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(m)
}

pub async fn edit(p: &PgPool, m: &subject::Subject) -> Result<u64> {
    if m.id.is_empty() {
        return Err(anyhow!("未指定ID").into());
    }

    let mut tx = p.begin().await.map_err(Error::from)?;

    let slug_exists =
        match subject::Subject::slug_is_exists(&mut *tx, &m.slug, Some(m.id.clone())).await {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(e.into());
            }
        };

    if slug_exists {
        return Err(anyhow!("slug已存在").into());
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

#[cfg(test)]
mod test {
    use sqlx::{postgres::PgPoolOptions, PgPool, Result};

    use crate::model;

    async fn get_pool() -> Result<PgPool> {
        let dsn = std::env::var("DB_DSN").unwrap();
        PgPoolOptions::new().max_connections(1).connect(&dsn).await
    }

    #[tokio::test]
    async fn test_add_subject() {
        let p = get_pool().await.unwrap();
        let m = model::subject::Subject {
            name: format!("专题-{}", 0),
            slug: format!("subject-{}", 0),
            summary: format!("专题摘要-{}", 0),
            ..Default::default()
        };
        let m = super::add(&p, m).await.unwrap();
        println!("{:?}", m);
    }

    #[tokio::test]
    async fn test_batch_add_subject() {
        let p = get_pool().await.unwrap();
        for i in 1..10 {
            let m = model::subject::Subject {
                name: format!("专题-{}", i),
                slug: format!("subject-{}", i),
                summary: format!("专题摘要-{}", i),
                ..Default::default()
            };
            let m = super::add(&p, m).await.unwrap();
            println!("{:?}", m);
        }
    }

    #[tokio::test]
    async fn test_edit_subject() {
        let p = get_pool().await.unwrap();
        let m = model::subject::Subject {
            id: "crpnr6kdrfart0b9j8u0".into(),
            name: format!("专题-{}", 0),
            slug: format!("subject-{}", 0),
            summary: format!("专题摘要-{}", 0),
            ..Default::default()
        };
        let aff = super::edit(&p, &m).await.unwrap();
        assert!(aff > 0);
    }
}
