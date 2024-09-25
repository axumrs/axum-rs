use anyhow::anyhow;
use sqlx::{PgExecutor, PgPool, QueryBuilder};

use crate::{
    model::topic::{Topic, TopicSection},
    utils, Error, Result,
};

// pub async fn add_sections()

pub async fn exists<'a>(
    c: impl PgExecutor<'a>,
    slug: &str,
    subject_id: &str,
    id: Option<&str>,
) -> sqlx::Result<bool> {
    let mut q = QueryBuilder::new("SELECT COUNT(*) FROM topics WHERE slug=");
    q.push_bind(slug)
        .push(" AND subject_id=")
        .push_bind(subject_id);

    if let Some(v) = id {
        q.push(" AND id<>").push_bind(v);
    }

    let count: (i64,) = q.build_query_as().fetch_one(c).await?;

    Ok(count.0 > 0)
}

pub async fn add(p: &PgPool, m: Topic, tag_names: &[&str]) -> Result<Topic> {
    let id = utils::id::new();
    let m = Topic { id, ..m };

    let mut tx = p.begin().await.map_err(Error::from)?;

    let exists = match exists(&mut *tx, &m.slug, &m.subject_id, None).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    if exists {
        return Err(anyhow!("同一专题下，相同的Slug已存在").into());
    }

    // 文章
    if let Err(e) = m.insert(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    // 段落

    // 标签
    let mut tag_ids = Vec::with_capacity(tag_names.len());
    for &tag_name in tag_names {
        let tag_id = match super::tag::insert_if_not_exists(&mut *tx, tag_name).await {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(e.into());
            }
        };
        tag_ids.push(tag_id);
    }
    // 文章-标签

    tx.commit().await.map_err(Error::from)?;

    Ok(m)
}
