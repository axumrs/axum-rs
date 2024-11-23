use anyhow::anyhow;
use sqlx::PgPool;

use crate::{model, utils, Error, Result};

pub async fn add(p: &PgPool, m: model::admin::Admin) -> Result<model::admin::Admin> {
    let id = utils::id::new();
    let m = model::admin::Admin { id, ..m };

    let mut tx = p.begin().await.map_err(Error::from)?;

    let exists = match model::admin::Admin::username_is_exists(&mut *tx, &m.username, None).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if exists {
        return Err(anyhow!("管理员已存在").into());
    }

    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    tx.commit().await.map_err(Error::from)?;

    Ok(m)
}
#[cfg(test)]
mod test {
    use sqlx::{postgres::PgPoolOptions, PgPool, Result};

    use crate::{model, utils};

    async fn get_pool() -> Result<PgPool> {
        let dsn = std::env::var("DB_DSN").unwrap();
        PgPoolOptions::new().max_connections(1).connect(&dsn).await
    }

    #[tokio::test]
    async fn test_add_admin() {
        let p = get_pool().await.unwrap();
        let username = "root".to_string();
        let password = utils::password::hash("axum.rs").unwrap();
        println!("password len: {}", password.len());
        let m = model::admin::Admin {
            username,
            password,
            ..Default::default()
        };
        let m = super::add(&p, m).await.unwrap();
        assert_eq!(m.id.is_empty(), false);
    }
}
