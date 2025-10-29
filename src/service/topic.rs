use anyhow::anyhow;
use chrono::Local;
use rand::Rng;
use sqlx::{PgExecutor, PgPool, QueryBuilder};

use crate::{
    config,
    model::{self, topic::Topic, topic_tag::TopicTag},
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

pub async fn edit(p: &PgPool, m: &Topic, tag_names: &[&str], hash_secret_key: &str) -> Result<u64> {
    if m.id.is_empty() {
        return Err(anyhow!("未指定ID").into());
    }

    let mut tx = p.begin().await.map_err(Error::from)?;

    let exists = match exists(&mut *tx, &m.slug, &m.subject_id, Some(&m.id)).await {
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
    if let Err(e) = m.update(&mut *tx).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    // -- 段落 --
    // 清空段落
    if let Err(e) = super::topic_section::clean(&mut *tx, &m.id).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }

    let sects = match utils::topic::sections(&m, hash_secret_key) {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(e.into());
        }
    };

    // 段落入库
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
    // 清空文章标签
    if let Err(e) = super::topic_tag::clean(&mut *tx, &m.id).await {
        tx.rollback().await.map_err(Error::from)?;
        return Err(e.into());
    }
    // 文章标签入库
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

    Ok(0)
}

async fn tags(
    p: &PgPool,
    tf: &model::topic_tag::VTopicTagWithTagListAllFilter,
) -> Result<Vec<model::topic_tag::VTopicTagWithTag>> {
    model::topic_tag::VTopicTagWithTag::list_all(p, tf)
        .await
        .map_err(Error::from)
}
pub async fn find_opt(
    p: &PgPool,
    f: Option<&model::topic_views::VTopicSubjectFindFilter>,
    tf: &model::topic_tag::VTopicTagWithTagListAllFilter,
    ts: Option<Option<model::topic_views::VTopicSubject>>,
    hidd_md: bool,
) -> Result<Option<model::topic_views::TopicSubjectWithTags>> {
    let topic_subjects = if let Some(ts) = ts {
        ts
    } else {
        model::topic_views::VTopicSubject::find(p, f.unwrap()).await?
    };
    let topic_subjects = match topic_subjects {
        Some(v) => {
            if hidd_md {
                model::topic_views::VTopicSubject {
                    md: "<HIDDEN>".to_string(),
                    ..v
                }
            } else {
                v
            }
        }
        None => return Ok(None),
    };

    let tags = tags(p, tf).await?;
    Ok(Some(model::topic_views::TopicSubjectWithTags {
        topic_subjects,
        tags,
    }))
}

pub async fn list_all_opt(
    p: &PgPool,
    f: &model::topic_views::VTopicSubjectListAllFilter,
) -> Result<Vec<model::topic_views::TopicSubjectWithTags>> {
    let tss = model::topic_views::VTopicSubject::list_all(p, f).await?;
    let mut r = Vec::with_capacity(tss.len());
    for ts in tss.into_iter() {
        let tf = model::topic_tag::VTopicTagWithTagListAllFilter {
            limit: None,
            order: None,
            topic_id: ts.id.clone(),
            name: None,
            is_del: Some(false),
        };
        let tst = find_opt(p, None, &tf, Some(Some(ts)), true).await?;
        if let Some(tst) = tst {
            r.push(tst);
        }
    }
    Ok(r)
}

pub async fn list_all_for_subject(
    p: &PgPool,
    subject_slug: String,
) -> Result<Vec<model::topic_views::TopicSubjectWithTags>> {
    let f = model::topic_views::VTopicSubjectListAllFilter {
        limit: None,
        order: Some("id ASC".into()),
        title: None,
        subject_id: None,
        slug: None,
        is_del: Some(false),
        subject_slug: Some(subject_slug),
        subject_is_del: Some(false),
        status: None,
        v_topic_subject_list_all_between_datelines: None,
    };

    list_all_opt(p, &f).await
}

/// 分页显示
pub async fn list_opt(
    p: &PgPool,
    f: &model::topic_views::VTopicSubjectListFilter,
    hidd_md: bool,
) -> Result<model::pagination::Paginate<model::topic_views::TopicSubjectWithTags>> {
    let tsp = model::topic_views::VTopicSubject::list(p, f).await?;

    let data = if hidd_md {
        tracing::debug!("hidd md");
        tsp.data
            .into_iter()
            .map(|ts| model::topic_views::VTopicSubject {
                md: "<HIDDEN>".to_string(),
                ..ts
            })
            .collect()
    } else {
        tracing::debug!("NOT hidd md");
        for d in &tsp.data {
            tracing::debug!("md len: {}", d.md.len());
        }
        tsp.data
    };
    let tsp = model::topic_views::VTopicSubjectPaginate {
        total: tsp.total,
        total_page: tsp.total_page,
        page: tsp.page,
        page_size: tsp.page_size,
        data,
    };

    let mut r = Vec::with_capacity(tsp.data.len());
    for ts in tsp.data.into_iter() {
        let tf = model::topic_tag::VTopicTagWithTagListAllFilter {
            limit: None,
            order: None,
            topic_id: ts.id.clone(),
            name: None,
            is_del: Some(false),
        };
        let tst = find_opt(p, None, &tf, Some(Some(ts)), hidd_md).await?;
        if let Some(tst) = tst {
            r.push(tst);
        }
    }
    Ok(model::pagination::Paginate {
        total: tsp.total,
        total_page: tsp.total_page,
        page: tsp.page,
        page_size: tsp.page_size,
        data: r,
    })
}

pub async fn find_detail(
    p: &PgPool,
    slug: &str,
    subject_slug: &str,
) -> Result<model::topic_views::TopicSubjectWithTagsAndSections> {
    let mut tx = p.begin().await?;
    let topic = match model::topic_views::VTopicSubject::find(
        &mut *tx,
        &model::topic_views::VTopicSubjectFindFilter {
            id: None,
            subject_id: None,
            slug: Some(slug.into()),
            is_del: Some(false),
            subject_slug: Some(subject_slug.into()),
            subject_is_del: Some(false),
        },
    )
    .await?
    {
        Some(v) => model::topic_views::VTopicSubject {
            md: "<HIDDEN>".to_string(),
            ..v
        },
        None => return Err(Error::new("文章不存在")),
    };

    // 更新阅读数
    let sql = format!(
        "UPDATE {} SET hit=hit+1 WHERE id = $1",
        &model::topic::Topic::table()
    );
    if let Err(e) = sqlx::query(&sql).bind(&topic.id).execute(&mut *tx).await {
        tx.rollback().await?;
        return Err(e.into());
    }

    // 标签

    let tags = match model::topic_tag::VTopicTagWithTag::list_all(
        &mut *tx,
        &model::topic_tag::VTopicTagWithTagListAllFilter {
            limit: None,
            order: None,
            topic_id: topic.id.clone(),
            name: None,
            is_del: Some(false),
        },
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    // 段落
    let sections = match model::topic::TopicSection::list_all(
        &mut *tx,
        &model::topic::TopicSectionListAllFilter {
            limit: None,
            order: Some("sort ASC, id ASC".into()),
            topic_id: Some(topic.id.clone()),
        },
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    tx.commit().await?;

    Ok(model::topic_views::TopicSubjectWithTagsAndSections {
        topic_subject_with_tags: model::topic_views::TopicSubjectWithTags {
            topic_subjects: topic,
            tags,
        },
        sections,
    })
}

pub async fn gen_protected_content(
    c: impl PgExecutor<'_>,
    secs: Vec<model::topic::TopicSection>,
    cfg: &config::ProtectedContentConfig,
) -> Result<(Vec<model::topic::TopicSection>, Vec<String>)> {
    if secs.len() <= cfg.min_sections as usize {
        return Ok((secs, vec![]));
    }

    // 保护的数量
    let procted_num = match secs.len() {
        0..=2 => 0,
        3..=5 => 1,
        6..=8 => 2,
        _ => cfg.max_sections as usize,
    };

    // 随机选择
    let mut procted_idx = Vec::with_capacity(procted_num);
    for _ in 0..procted_num {
        loop {
            let idx = rand::rng().random_range(0..secs.len());
            if !utils::vec::is_in(&procted_idx, &idx) {
                procted_idx.push(idx);
                break;
            }
        }
    }
    procted_idx.sort();

    let procted_contents = secs
        .iter()
        .enumerate()
        .filter(|(idx, _)| utils::vec::is_in(&procted_idx, idx))
        .map(|(_, sect)| {
            let id = utils::id::new();
            let expire_time = Local::now() + chrono::Duration::minutes(cfg.timeout as i64);
            model::protected_content::ProtectedContent {
                id,
                section_id: sect.id.clone(),
                content: sect.content.clone(),
                expire_time,
            }
        })
        .collect::<Vec<_>>();

    let mut q = QueryBuilder::new(
        r#"INSERT INTO protected_contents (id, section_id, "content", expire_time) "#,
    );
    q.push_values(&procted_contents, |mut b, pc| {
        b.push_bind(&pc.id)
            .push_bind(&pc.section_id)
            .push_bind(&pc.content)
            .push_bind(&pc.expire_time);
    });

    q.build().execute(c).await?;

    let procted_content_ids = procted_contents
        .iter()
        .map(|pc| pc.section_id.clone())
        .collect::<Vec<_>>();

    let secs_r = secs
        .into_iter()
        .map(|s| {
            if utils::vec::is_in(&procted_content_ids, &s.id) {
                let pid = &procted_contents
                    .iter()
                    .find(|pc| pc.section_id == s.id)
                    .unwrap()
                    .id;
                let content = format!(
                    r#"<div data-procted-id="{}">{}</div>"#,
                    pid, &cfg.placeholder
                );
                model::topic::TopicSection { content, ..s }
            } else {
                s
            }
        })
        .collect::<Vec<_>>();
    Ok((
        secs_r,
        procted_contents
            .into_iter()
            .map(|pc| pc.id)
            .collect::<Vec<_>>(),
    ))
}

pub async fn get_protected_content(
    p: &PgPool,
    ids: &[String],
) -> Result<Vec<model::protected_content::ProtectedContent>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let mut q = QueryBuilder::new(
        r#"SELECT id, section_id, "content", expire_time FROM protected_contents WHERE id IN "#,
    );
    q.push_tuples(ids, |mut b, id| {
        b.push_bind(id);
    });

    q.build_query_as().fetch_all(p).await.map_err(Error::from)
}

pub async fn gen_guess_content(
    secs: Vec<model::topic::TopicSection>,
    cfg: &config::ProtectedContentConfig,
    is_need_buy: bool,
) -> Result<Vec<model::topic::TopicSection>> {
    if is_need_buy {
        return Ok(vec![]);
    }
    if secs.len() <= cfg.min_sections as usize {
        return Ok(secs);
    }

    // 保护的数量
    let procted_num = match secs.len() {
        0..=2 => 0,
        3..=4 => 1,
        5..=7 => 2,
        8..=10 => 3,
        _ => 5,
    };

    // 随机选择
    let mut procted_idx = Vec::with_capacity(procted_num);
    for _ in 0..procted_num {
        loop {
            let idx = rand::rng().random_range(0..secs.len());
            if !utils::vec::is_in(&procted_idx, &idx) {
                procted_idx.push(idx);
                break;
            }
        }
    }
    procted_idx.sort();

    let mut guess_sects = vec![];
    for (idx, s) in secs.into_iter().enumerate() {
        if !utils::vec::is_in(&procted_idx, &idx) {
            guess_sects.push(s)
        } else {
            let content = format!(r#"<div data-guess-id="{}"></div>"#, s.id.clone());
            guess_sects.push(model::topic::TopicSection { content, ..s });
        }
    }
    Ok(guess_sects)
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

    #[tokio::test]
    async fn test_find_opt_topic() {
        let id = "crv55gkdrfanbmmathl0".to_string();
        let p = get_pool().await.unwrap();

        let f = model::topic_views::VTopicSubjectFindFilter {
            id: Some(id.clone()),
            subject_id: None,
            slug: None,
            is_del: Some(false),
            subject_slug: None,
            subject_is_del: Some(false),
        };
        let tf = model::topic_tag::VTopicTagWithTagListAllFilter {
            limit: None,
            order: None,
            topic_id: id.clone(),
            name: None,
            is_del: Some(false),
        };
        let ls = super::find_opt(&p, Some(&f), &tf, None, true)
            .await
            .unwrap();
        println!("{:#?}", ls);
    }

    #[tokio::test]
    async fn test_list_all_opt_topic() {
        let p = get_pool().await.unwrap();
        let f = model::topic_views::VTopicSubjectListAllFilter {
            limit: None,
            order: Some("id ASC".into()),
            title: None,
            subject_id: Some("crv55gkdrfanbmmatc6g".into()),
            slug: None,
            is_del: Some(false),
            subject_slug: None,
            subject_is_del: Some(false),
            status: None,
            v_topic_subject_list_all_between_datelines: None,
        };
        let ls = super::list_all_opt(&p, &f).await.unwrap();
        println!("{:#?}", ls);
    }
}
