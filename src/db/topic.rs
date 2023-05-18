use crate::{model, Error, Result};

use super::Paginate;

pub async fn exists<'a, C>(
    conn: C,
    slug: &'a str,
    subject_id: &'a u32,
    id: Option<&'a u64>,
) -> Result<bool>
where
    C: sqlx::MySqlExecutor<'a>,
{
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM topic WHERE slug = ");
    q.push_bind(slug);

    q.push(" AND subject_id = ");
    q.push_bind(subject_id);

    if let Some(id) = id {
        q.push(" AND id<> ");
        q.push_bind(id);
    }
    let count: (i64,) = q
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    Ok(count.0 > 0)
}

pub async fn add(
    conn: &sqlx::MySqlPool,
    m: &model::Topic,
    c: &model::TopicContent,
    tag_ids: Option<Vec<u32>>,
) -> Result<u64> {
    let mut tx = conn.begin().await.map_err(Error::from)?;
    if exists(&mut tx, &m.slug, &m.subject_id, None).await? {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::already_exists("同一专题下，相同的slug已存在"));
    }

    // 主表
    let id = match sqlx::query("INSERT INTO topic (title, subject_id, slug, summary, author, src, hit, dateline, try_readable, is_del,cover) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?,?)")
    .bind(&m.title)
    .bind(&m.subject_id)
    .bind(&m.slug)
    .bind(&m.summary)
    .bind(&m.author)
    .bind(&m.src)
    .bind(&m.hit)
    .bind(&m.dateline)
    .bind(&m.try_readable)
    .bind(&m.is_del)
    .bind(&m.cover)
    .execute(&mut tx).await {
        Ok(q) => q.last_insert_id(),
        Err(err) => {
             tx.rollback().await.map_err(Error::from)?;
             return Err(Error::from(err));
        }
    };

    // 内容表
    if let Err(err) = sqlx::query("INSERT INTO topic_content (topic_id, md, html) VALUES(?, ?, ?)")
        .bind(&id)
        .bind(&c.md)
        .bind(&c.html)
        .execute(&mut tx)
        .await
    {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::from(err));
    }

    // 标签
    // TODO: 单条SQL批量插入
    if let Some(tag_ids) = tag_ids {
        for tag_id in tag_ids {
            if let Err(err) = sqlx::query("INSERT INTO topic_tag (topic_id,tag_id) VALUES (?,?)")
                .bind(id)
                .bind(tag_id)
                .execute(&mut tx)
                .await
            {
                tx.rollback().await.map_err(Error::from)?;
                return Err(Error::from(err));
            }
        }
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(id)
}
pub async fn edit(
    conn: &sqlx::MySqlPool,
    m: &model::Topic,
    c: &model::TopicContent,
    tag_ids: Option<Vec<u32>>,
) -> Result<u64> {
    let mut tx = conn.begin().await.map_err(Error::from)?;
    if exists(&mut tx, &m.slug, &m.subject_id, Some(&m.id)).await? {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::already_exists("同一专题下，相同的slug已存在"));
    }

    // 主表
    let aff = match sqlx::query("UPDATE topic SET title=?, subject_id=?, slug=?, summary=?, author=?, src=?, try_readable=?,cover=? WHERE id=?")
    .bind(&m.title)
    .bind(&m.subject_id)
    .bind(&m.slug)
    .bind(&m.summary)
    .bind(&m.author)
    .bind(&m.src)
    .bind(&m.try_readable)
    .bind(&m.cover)
    .bind(&m.id)
    .execute(&mut tx).await {
        Ok(q) => q.rows_affected(),
        Err(err) => {
             tx.rollback().await.map_err(Error::from)?;
             return Err(Error::from(err));
        }
    };

    // 内容表
    if let Err(err) = sqlx::query("UPDATE topic_content SET md=?, html=? WHERE topic_id=?")
        .bind(&c.md)
        .bind(&c.html)
        .bind(&m.id)
        .execute(&mut tx)
        .await
    {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::from(err));
    }

    // 标签
    // TODO: 单条SQL批量插入
    if let Some(tag_ids) = tag_ids {
        // 清空已有标签
        if let Err(err) = sqlx::query("DELETE FROM topic_tag WHERE topic_id=?")
            .bind(&m.id)
            .execute(&mut tx)
            .await
        {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
        for tag_id in tag_ids {
            if let Err(err) = sqlx::query("INSERT INTO topic_tag (topic_id,tag_id) VALUES (?,?)")
                .bind(&m.id)
                .bind(tag_id)
                .execute(&mut tx)
                .await
            {
                tx.rollback().await.map_err(Error::from)?;
                return Err(Error::from(err));
            }
        }
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(aff)
}

pub async fn list2admin(
    conn: &sqlx::MySqlPool,
    with: &model::Topic2AdminListWith,
) -> Result<Paginate<model::Topic2AdminList>> {
    let mut q = sqlx::QueryBuilder::new(
        r"SELECT id, title, slug, hit, dateline, try_readable, is_del, cover, subject_name, subject_slug FROM v_topic_admin_list WHERE 1=1",
    );
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_topic_admin_list WHERE 1=1");

    if let Some(title) = &with.title {
        let sql = " AND title LIKE ";
        let arg = format!("%{}%", title);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }
    if let Some(slug) = &with.slug {
        let sql = " AND slug LIKE ";
        let arg = format!("%{}%", slug);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }
    if let Some(subject_name) = &with.subject_name {
        let sql = " AND subject_name LIKE ";
        let arg = format!("%{}%", subject_name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }
    if let Some(try_readable) = &with.try_readable {
        let sql = " AND try_readable=";

        q.push(sql).push_bind(try_readable);
        qc.push(sql).push_bind(try_readable);
    }
    if let Some(is_del) = &with.is_del {
        let sql = " AND is_del=";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    q.push(" ORDER BY id DESC ")
        .push(" LIMIT ")
        .push_bind(with.page_size)
        .push(" OFFSET ")
        .push_bind(with.page * with.page_size);

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;
    let data = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(Paginate::new(
        count.0 as u32,
        with.page,
        with.page_size,
        data,
    ))
}

pub async fn del_or_restore(conn: &sqlx::MySqlPool, id: u64, is_del: bool) -> Result<u64> {
    super::del_or_restore(
        conn,
        "topic",
        super::DelOrRestorePrimaryKey::BigInt(id),
        is_del,
    )
    .await
}

pub async fn find2edit(conn: &sqlx::MySqlPool, id: u64) -> Result<Option<model::Topic2Edit>> {
    let r = sqlx::query_as("SELECT id, title, subject_id, slug, summary, author, src, try_readable, cover,md FROM topic AS t INNER JOIN topic_content AS tc ON t.id=tc.topic_id WHERE id=?").bind(id).fetch_optional(conn).await.map_err(Error::from)?;
    Ok(r)
}

pub async fn get_tags(conn: &sqlx::MySqlPool, id: u64) -> Result<Vec<model::Tag2TopicEdit>> {
    let r = sqlx::query_as("SELECT t.name FROM tag as t INNER JOIN topic_tag as tt ON t.id=tt.tag_id WHERE tt.topic_id =? AND tt.is_del=?").bind(id).bind(false).fetch_all(conn).await.map_err(Error::from)?;

    Ok(r)
}

pub async fn list2web(
    conn: &sqlx::MySqlPool,
    with: &model::Topic2WebListWith,
) -> Result<Paginate<model::Topic2WebList>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, title, slug, try_readable, cover, summary, subject_name, subject_slug, tag_names FROM v_topic_web_list WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_topic_web_list WHERE 1=1");

    if let Some(title) = &with.title {
        let sql = " AND title LIKE";
        let arg = format!("%{}%", title);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(subject_name) = &with.subject_name {
        let sql = " AND subject_name LIKE";
        let arg = format!("%{}%", subject_name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }
    if let Some(subject_slug) = &with.subject_slug {
        let sql = " AND subject_slug =";

        q.push(sql).push_bind(subject_slug);
        qc.push(sql).push_bind(subject_slug);
    }

    let order_by = if with.order_by_hit {
        " ORDER BY hit DESC"
    } else {
        " ORDER BY id DESC"
    };

    q.push(order_by)
        .push(" LIMIT ")
        .push_bind(with.page_size)
        .push(" OFFSET ")
        .push_bind(with.page * with.page_size);

    let count: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    let data = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(Paginate::new(
        count.0 as u32,
        with.page,
        with.page_size,
        data,
    ))
}
