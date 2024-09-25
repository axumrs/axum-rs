use anyhow::anyhow;
use sqlx::{PgExecutor, PgPool, QueryBuilder};

use crate::{
    model::{topic::Topic, topic_tag::TopicTag},
    utils, Error, Result,
};

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

pub async fn add(p: &PgPool, m: Topic, tag_names: &[&str], hash_secret_key: &str) -> Result<Topic> {
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
    let sects = match utils::topic::sections(&m, hash_secret_key) {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    for sect in sects.into_iter() {
        if let Err(e) = sect.insert(&mut *tx).await {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    }

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
    let topic_tags = tag_ids
        .iter()
        .map(|tag_id| TopicTag {
            id: utils::id::new(),
            topic_id: m.id.clone(),
            tag_id: tag_id.to_owned(),
        })
        .collect::<Vec<_>>();
    for topic_tag in topic_tags.into_iter() {
        if let Err(e) = topic_tag.insert(&mut *tx).await {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    }

    tx.commit().await.map_err(Error::from)?;

    Ok(m)
}

#[cfg(test)]
mod test {
    use sqlx::{postgres::PgPoolOptions, PgPool, Result};

    use crate::model;

    async fn get_pool() -> Result<PgPool> {
        let dsn = std::env::var("DB_DSN").unwrap();
        PgPoolOptions::new().max_connections(1).connect(&dsn).await
    }

    fn read_data(filename: &str) -> std::io::Result<String> {
        let path = format!("test-sections-data/{}", filename);
        std::fs::read_to_string(&path)
    }

    #[tokio::test]
    async fn test_add_topic() {
        let p = get_pool().await.unwrap();
        let md = read_data("b.md").unwrap();
        let topic = model::topic::Topic {
            title: format!("etcd的基础知识"),
            subject_id: "crpnr6kdrfart0b9j8u0".into(),
            slug: "etcd-basic".into(),
            md,
            ..Default::default()
        };
        let hash_secret_key = "";
        let tag_names = &["etcd", "KV", "配置", "异步"];
        let m = super::add(&p, topic, tag_names, hash_secret_key)
            .await
            .unwrap();
        println!("{:#?}", m);
    }
}
