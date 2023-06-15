use crate::{model, Error, Result};

/// slug 是否存在
pub async fn exists(conn: &sqlx::MySqlPool, slug: &str, id: Option<u32>) -> Result<bool> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM subject WHERE slug=");
    q.push_bind(slug);

    if let Some(id) = id {
        q.push(" AND id <> ");
        q.push_bind(id);
    }
    let row: (i64,) = q
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;
    Ok(row.0 > 0)
}

/// 添加专题
pub async fn add(conn: &sqlx::MySqlPool, m: &model::Subject) -> Result<u32> {
    if exists(conn, &m.slug, None).await? {
        return Err(Error::already_exists("相同的slug已存在"));
    }

    let id = sqlx::query("INSERT INTO subject (name, slug, summary, is_del, cover, status, price,pin) VALUES(?,?,?,?,?,?,?,?)")
    .bind(&m.name)
    .bind(&m.slug)
    .bind(&m.summary)
    .bind(false)
    .bind(&m.cover)
    .bind(&m.status)
    .bind(&m.price)
    .bind(&m.pin)
    .execute(conn).await.map_err(Error::from)?
    .last_insert_id();
    Ok(id as u32)
}

/// 删除或还原
pub async fn del_or_restore(conn: &sqlx::MySqlPool, id: u32, is_del: bool) -> Result<u64> {
    super::del_or_restore(
        conn,
        "subject",
        super::DelOrRestorePrimaryKey::Int(id),
        is_del,
    )
    .await
}

/// 更新
pub async fn update(conn: &sqlx::MySqlPool, m: &model::Subject) -> Result<u64> {
    if exists(conn, &m.slug, Some(m.id)).await? {
        return Err(Error::already_exists("同名的slug已存在"));
    }
    let r = sqlx::query(
        "UPDATE subject SET name=?, slug=?, summary=?, status=?, price=?, cover=?,pin=? WHERE id=?",
    )
    .bind(&m.name)
    .bind(&m.slug)
    .bind(&m.summary)
    .bind(&m.status)
    .bind(&m.price)
    .bind(&m.cover)
    .bind(&m.pin)
    .bind(&m.id)
    .execute(conn)
    .await
    .map_err(Error::from)?
    .rows_affected();
    Ok(r)
}

/// 查找单条记录
pub async fn find<'a>(
    conn: &sqlx::MySqlPool,
    by: model::SubjectFindBy<'a>,
    is_del: Option<bool>,
) -> Result<Option<model::Subject>> {
    let mut q = sqlx::QueryBuilder::new(
        "SELECT id,name, slug, summary, is_del, cover, status, price,pin FROM subject WHERE 1=1",
    );

    match by {
        model::SubjectFindBy::ID(id) => {
            q.push(" AND id=").push_bind(id);
        }
        model::SubjectFindBy::Slug(slug) => {
            q.push(" AND slug=").push_bind(slug);
        }
    }

    if let Some(is_del) = is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }
    q.push(" LIMIT 1");

    let r = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;
    Ok(r)
}

/// 分页列表
pub async fn list(
    conn: &sqlx::MySqlPool,
    with: model::SubjectListWith,
) -> Result<super::Paginate<model::Subject>> {
    let mut q = sqlx::QueryBuilder::new(
        "SELECT id,name, slug, summary, is_del, cover, status, price,pin FROM subject WHERE 1=1",
    );
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM subject WHERE 1=1");

    if let Some(name) = &with.name {
        let sql = " AND name LIKE ";
        let arg = format!("%{}%", name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(slug) = &with.slug {
        let sql = " AND slug LIKE ";
        let arg = format!("%{}%", slug);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(status) = &with.status {
        let sql = " AND status = ";

        q.push(sql).push_bind(status);
        qc.push(sql).push_bind(status);
    }

    if let Some(is_del) = &with.is_del {
        let sql = " AND is_del = ";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    q.push(" ORDER BY pin DESC,id DESC ");

    q.push(" LIMIT ")
        .push_bind(with.page_size)
        .push(" OFFSET ")
        .push_bind(with.page * with.page_size);

    let total: (i64,) = qc
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    let data = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(super::Paginate::new(
        total.0 as u32,
        with.page,
        with.page_size,
        data,
    ))
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::{model, Error, Result};

    async fn get_conn() -> Result<sqlx::MySqlPool> {
        let dsn = env::var("MYSQL_DSN")
            .unwrap_or("mysql://root:root@127.0.0.1:23306/axum_rs".to_string());
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&dsn)
            .await
            .map_err(Error::from)?;
        Ok(pool)
    }

    #[tokio::test]
    async fn test_subject_slug_exists() {
        let pool = get_conn().await.unwrap();
        let exists = super::exists(&pool, "axum-rs", None).await.unwrap();
        assert!(exists == false);
    }

    #[tokio::test]
    async fn test_add_subject() {
        let pool = get_conn().await.unwrap();
        let m = model::Subject {
            name: format!("专题0"),
            slug: format!("subject-0"),
            summary: format!("专题0的说明"),
            ..Default::default()
        };
        let id = super::add(&pool, &m).await.unwrap();
        assert!(id > 0);
    }

    #[tokio::test]
    async fn test_batch_add_subject() {
        let pool = get_conn().await.unwrap();
        for i in 1..=30 {
            let status = if i % 2 == 0 {
                model::SubjectStatus::Finished
            } else {
                model::SubjectStatus::Writing
            };

            let m = model::Subject {
                name: format!("专题{}", i),
                slug: format!("subject-{}", i),
                summary: format!("专题{}的说明", i),
                status,
                price: i * 100,
                ..Default::default()
            };
            let id = super::add(&pool, &m).await.unwrap();
            assert!(id > 0);
        }
    }

    #[tokio::test]
    async fn test_del_subject() {
        let pool = get_conn().await.unwrap();
        let aff = super::del_or_restore(&pool, 1, true).await.unwrap();
        assert!(aff > 0);
    }

    #[tokio::test]
    async fn test_restore_subject() {
        let pool = get_conn().await.unwrap();
        let aff = super::del_or_restore(&pool, 1, false).await.unwrap();
        assert!(aff > 0);
    }

    #[tokio::test]
    async fn test_update_subject() {
        let pool = get_conn().await.unwrap();
        let m = model::Subject {
            id: 1,
            name: format!("专题0!!!"),
            slug: format!("subject-0"),
            summary: format!("专题0的说明!"),
            cover: format!("//axum.rs/asset/bg.png"),
            ..Default::default()
        };
        let id = super::update(&pool, &m).await.unwrap();
        assert!(id > 0);
    }

    #[tokio::test]
    async fn test_find_subject() {
        let pool = get_conn().await.unwrap();
        // let by = model::SubjectFindBy::ID(1);
        let by = model::SubjectFindBy::Slug("subject-0");
        let is_del: Option<bool> = None;
        let m = super::find(&pool, by, is_del).await.unwrap();
        assert!(m.is_some());
        let m = m.unwrap();
        assert!(m.id == 1);
    }

    #[tokio::test]
    async fn test_list_subjects() {
        let pool = get_conn().await.unwrap();
        let with = model::SubjectListWith {
            page: 0,
            page_size: 3,
            name: Some("专题1".to_string()),
            slug: Some("subject-1".to_string()),
            is_del: Some(false),
            status: Some(model::SubjectStatus::Finished),
            ..Default::default()
        };
        let p = super::list(&pool, with).await.unwrap();
        println!("paginate: {}, {}", p.total, p.total_page);
        for s in p.data {
            println!("{:?}", s);
        }
    }
}
