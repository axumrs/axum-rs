use crate::{model, password, Error, Result};

use super::Paginate;

pub async fn exists(conn: &sqlx::MySqlPool, username: &str, id: Option<u32>) -> Result<bool> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM admin WHERE username=");
    q.push_bind(username);

    if let Some(id) = id {
        q.push(" AND id<>").push_bind(id);
    }
    let count: (i64,) = q
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;
    Ok(count.0 > 0)
}

pub async fn add(conn: &sqlx::MySqlPool, admin: &model::Admin) -> Result<u32> {
    if exists(conn, &admin.username, None).await? {
        return Err(Error::already_exists("管理员已存在"));
    }

    let pwd = password::hash(&admin.password)?;

    let id = sqlx::query("INSERT INTO admin (username, password) VALUES(?,?)")
        .bind(&admin.username)
        .bind(pwd)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .last_insert_id();
    Ok(id as u32)
}

pub async fn find<'a>(
    conn: &sqlx::MySqlPool,
    by: &model::AdminFindBy<'a>,
) -> Result<Option<model::Admin>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id,username,password,is_del FROM admin WHERE 1=1");
    match by {
        &model::AdminFindBy::ID(id) => q.push(" AND id=").push_bind(id),
        &model::AdminFindBy::Username(username) => q.push(" AND username=").push_bind(username),
    };
    q.push(" LIMIT 1");

    let a = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;

    Ok(a)
}

pub async fn list(
    conn: &sqlx::MySqlPool,
    with: &model::PaginateWith,
) -> Result<Paginate<model::Admin>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id,username,password,is_del FROM admin");
    q.push(" ORDER BY id DESC")
        .push(" LIMIT ")
        .push_bind(with.page_size)
        .push(" OFFSET ")
        .push_bind(with.page * with.page_size);

    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM admin");

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

pub async fn del_or_restore(conn: &sqlx::MySqlPool, id: u32, is_del: bool) -> Result<u64> {
    super::del_or_restore(
        conn,
        "admin",
        super::DelOrRestorePrimaryKey::Int(id),
        is_del,
    )
    .await
    .map_err(Error::from)
}

pub async fn edit(conn: &sqlx::MySqlPool, a: &model::Admin2Edit) -> Result<u64> {
    if exists(conn, &a.username, Some(a.id)).await? {
        return Err(Error::already_exists("管理员已经存在"));
    }
    let mut q = sqlx::QueryBuilder::new("UPDATE admin SET username=");
    q.push_bind(&a.username);

    if let Some(pwd) = &a.password {
        let pwd = password::hash(pwd)?;
        q.push(", password=").push_bind(pwd);
    }

    q.push(" WHERE id=").push_bind(a.id);

    let aff = q
        .build()
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}
