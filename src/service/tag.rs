use anyhow::anyhow;
use sqlx::{PgExecutor, PgPool};

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
            is_del: None,
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
            is_del: None,
        },
    )
    .await
}

pub async fn insert_if_not_exists<'a>(c: impl PgExecutor<'a>, name: &str) -> sqlx::Result<String> {
    let sql = r#"INSERT INTO tags (id, "name", is_del) VALUES ($1, $2, FALSE) ON CONFLICT ("name") DO UPDATE SET is_del=EXCLUDED.is_del RETURNING id"#;

    let gen_id = utils::id::new();
    let id: (String,) = sqlx::query_as(sql)
        .bind(&gen_id)
        .bind(name)
        .fetch_one(c)
        .await?;

    Ok(id.0)
}

pub async fn del(p: &PgPool, id: String) -> Result<(u64, u64)> {
    let mut tx = p.begin().await.map_err(Error::from)?;

    let tag_aff = match model::tag::Tag::real_del(&mut *tx, &id).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    let clean_topic_tag_aff = match super::topic_tag::clean_by_tag(&mut *tx, &id).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    tx.commit().await.map_err(Error::from)?;

    Ok((tag_aff, clean_topic_tag_aff))
}

pub async fn find_with_topic_count(
    p: &PgPool,
    f: Option<&model::tag::TagFindFilter>,
    m: Option<model::tag::Tag>,
) -> Result<Option<model::tag::TagWithTopicCount>> {
    let t = if let Some(v) = m {
        v
    } else {
        let f = match f {
            Some(v) => v,
            None => return Err(Error::new("参数错误")),
        };

        match model::tag::Tag::find(p, f).await? {
            Some(v) => v,
            None => return Ok(None),
        }
    };

    let c: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM topic_tags WHERE tag_id = $1")
        .bind(&t.id)
        .fetch_one(&*p)
        .await?;

    Ok(Some(model::tag::TagWithTopicCount {
        tag: t,
        topic_count: c.0,
    }))
}
pub async fn list_with_topic_count(
    p: &PgPool,
    page: u32,
    page_size: u32,
) -> Result<model::pagination::Paginate<model::tag::TagWithTopicCount>> {
    let tp = model::tag::Tag::list(
        &*p,
        &model::tag::TagListFilter {
            pq: model::tag::TagPaginateReq { page, page_size },
            order: None,
            name: None,
            is_del: Some(false),
        },
    )
    .await?;

    let mut r = Vec::with_capacity(tp.data.len());
    for t in tp.data {
        let m = find_with_topic_count(p, None, Some(t)).await?.unwrap();
        r.push(m);
    }

    Ok(model::pagination::Paginate {
        total: tp.total,
        total_page: tp.total_page,
        page: tp.page,
        page_size: tp.page_size,
        data: r,
    })
}

#[cfg(test)]
mod test {
    use sqlx::{postgres::PgPoolOptions, PgPool, Result};

    async fn get_pool() -> Result<PgPool> {
        let dsn = std::env::var("DB_DSN").unwrap();
        PgPoolOptions::new().max_connections(1).connect(&dsn).await
    }

    #[tokio::test]
    async fn test_insert_tag_if_not_exists() {
        let p = get_pool().await.unwrap();

        let name = "postgres";

        let id = super::insert_if_not_exists(&p, name).await.unwrap();
        println!("tag id: {}", id);
    }
    #[tokio::test]
    async fn test_batch_insert_tag_if_not_exists() {
        let p = get_pool().await.unwrap();

        let mut ids = vec![];
        let mut tx = p.begin().await.unwrap();
        for name in &["postgres", "axum", "异步"] {
            let id = match super::insert_if_not_exists(&mut *tx, name).await {
                Ok(v) => v,
                Err(e) => {
                    tx.rollback().await.unwrap();
                    panic!("{}", e);
                }
            };
            ids.push(id);
        }
        tx.commit().await.unwrap();
        println!("{:?}", ids);
    }
}
