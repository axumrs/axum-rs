use crate::{model, Error, Result};

pub async fn exists<'a, C>(conn: C, name: &'a str) -> Result<bool>
where
    C: sqlx::MySqlExecutor<'a>,
{
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tag WHERE name=?")
        .bind(name)
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;
    Ok(count.0 > 0)
}

pub async fn add(conn: &sqlx::MySqlPool, m: &model::Tag) -> Result<u32> {
    if exists(conn, &m.name).await? {
        return Err(Error::already_exists("相同的tag已存在"));
    }
    let id = sqlx::query("INSERT INTO tag (name) VALUES (?) ON DUPLICATE KEY UPDATE name=name")
        .bind(&m.name)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .last_insert_id();

    Ok(id as u32)
}

pub async fn find<'a>(
    conn: &sqlx::MySqlPool,
    by: &model::TagFindBy<'a>,
    is_del: Option<bool>,
) -> Result<Option<model::Tag>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id,name,is_del FROM tag WHERE 1=1");
    match by {
        &model::TagFindBy::ID(id) => {
            q.push(" AND id=").push_bind(id);
        }
        &model::TagFindBy::Name(name) => {
            let arg = format!("%{}%", name);
            q.push(" AND name LIKE ").push_bind(arg);
        }
        &model::TagFindBy::ExactName(name) => {
            q.push(" AND name = ").push_bind(name);
        }
    };
    if let Some(is_del) = is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }
    q.push(" LIMIT 1");

    let t = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;
    Ok(t)
}

pub async fn auto(conn: &sqlx::MySqlPool, tag_names: &Vec<String>) -> Result<Vec<u32>> {
    let mut ids = Vec::with_capacity(tag_names.len());

    for name in tag_names {
        let by = model::TagFindBy::ExactName(name);
        if let Some(tag) = find(conn, &by, None).await? {
            ids.push(tag.id);
            continue;
        }
        let m = model::Tag {
            name: name.to_owned(),
            ..Default::default()
        };
        let id = add(conn, &m).await?;
        // let id = 1u32;
        ids.push(id);
    }

    Ok(ids)
}

pub async fn list(
    conn: &sqlx::MySqlPool,
    with: &model::TagListWith,
) -> Result<super::Paginate<model::Tag>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id,name,is_del FROM tag WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM tag WHERE 1=1");
    if let Some(name) = &with.name {
        let sql = " AND name LIKE ";
        let arg = format!("%{}%", name);
        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }
    if let Some(is_del) = with.is_del {
        let sql = " AND is_del=";
        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    q.push(" ORDER BY id DESC")
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

    Ok(super::Paginate::new(
        count.0 as u32,
        with.page,
        with.page_size,
        data,
    ))
}

pub async fn del_or_restore(conn: &sqlx::MySqlPool, id: u32, is_del: bool) -> Result<u64> {
    super::del_or_restore(conn, "tag", super::DelOrRestorePrimaryKey::Int(id), is_del).await
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
    async fn test_add_tag() {
        let conn = get_conn().await.unwrap();
        let id = super::add(
            &conn,
            &model::Tag {
                name: format!("tag-1"),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        println!("id: {}", id);
        // assert!(id == 1);
    }
    #[tokio::test]
    async fn test_find_tag() {
        let conn = get_conn().await.unwrap();
        // let by = model::TagFindBy::ID(1);
        // let by = model::TagFindBy::Name("tag-");
        let by = model::TagFindBy::ExactName("tag-1");
        let t = super::find(&conn, &by, None).await.unwrap();

        assert!(t.is_some());
        assert!(t.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_auto_tag() {
        let conn = get_conn().await.unwrap();
        let tag_names = vec![
            "tag-1".to_string(),
            "tag-2".to_string(),
            "tag-3".to_string(),
            "tag-4".to_string(),
            "tag-5".to_string(),
            "tag-6".to_string(),
            "tag-10".to_string(),
            "tag-22".to_string(),
        ];
        let ids = super::auto(&conn, &tag_names).await.unwrap();
        println!("{:?}", ids);
    }

    #[tokio::test]
    async fn test_list_tag() {
        let conn = get_conn().await.unwrap();
        let with = model::TagListWith {
            page: 1,
            page_size: 3,
            ..Default::default()
        };
        let p = super::list(&conn, &with).await.unwrap();
        println!("{}, {}", p.total, p.total_page);
        for t in p.data {
            println!("{:?}", t);
        }
    }
}
