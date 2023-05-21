use crate::{model, password, Error, Result};

use super::Paginate;

pub async fn exists(conn: &sqlx::MySqlPool, email: &str, id: Option<u32>) -> Result<bool> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM `user` WHERE email=");
    q.push_bind(email);

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
pub async fn exists_nickname(
    conn: &sqlx::MySqlPool,
    nickname: &str,
    id: Option<u32>,
) -> Result<bool> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM `user` WHERE nickname=");
    q.push_bind(nickname);

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

pub async fn add(conn: &sqlx::MySqlPool, u: &model::User) -> Result<u32> {
    if exists(conn, &u.email, None).await? {
        return Err(Error::already_exists("Email已存在"));
    }
    if exists_nickname(conn, &u.nickname, None).await? {
        return Err(Error::already_exists("昵称已存在"));
    }

    let pwd = password::hash(&u.password)?;

    let id = sqlx::query("INSERT INTO `user` (email, nickname, password, status, dateline, types, sub_exp, points) VALUES(?,?,?,?,?,?,?,?);
")
    .bind(&u.email)
    .bind(&u.nickname)
    .bind(&pwd)
    .bind(&u.status)
    .bind(&u.dateline)
    .bind(&u.types)
    .bind(&u.sub_exp)
    .bind(&u.points)
    .execute(conn).await.map_err(Error::from)?.last_insert_id();

    Ok(id as u32)
}

pub async fn edit(conn: &sqlx::MySqlPool, u: &model::UserEdit2Admin) -> Result<u64> {
    if exists(conn, &u.email, Some(u.id)).await? {
        return Err(Error::already_exists("Email已存在"));
    }
    if exists_nickname(conn, &u.nickname, Some(u.id)).await? {
        return Err(Error::already_exists("昵称已存在"));
    }

    let mut q = sqlx::QueryBuilder::new("UPDATE `user` SET email=");
    q.push_bind(&u.email)
        .push(", nickname=")
        .push_bind(&u.nickname)
        .push(", status=")
        .push_bind(&u.status)
        .push(", types=")
        .push_bind(&u.types)
        .push(", sub_exp=")
        .push_bind(u.sub_exp)
        .push(", points=")
        .push_bind(&u.points);

    if let Some(pwd) = &u.password {
        let pwd = password::hash(pwd)?;
        q.push(", password=").push_bind(pwd);
    }
    q.push(" WHERE id=").push_bind(&u.id);

    let aff = q
        .build()
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn change_status(
    conn: &sqlx::MySqlPool,
    id: u32,
    status: &model::UserStatus,
) -> Result<u64> {
    let aff = sqlx::query("UPDATE `user` SET status=? WHERE id=?")
        .bind(status)
        .bind(id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn increment_points(conn: &sqlx::MySqlPool, id: u32, points: i32) -> Result<u64> {
    let aff = sqlx::query("UPDATE `user` SET points=points+? WHERE id=?")
        .bind(points)
        .bind(id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn del_or_restore(conn: &sqlx::MySqlPool, id: u32, is_del: bool) -> Result<u64> {
    super::del_or_restore(
        conn,
        "`user`",
        super::DelOrRestorePrimaryKey::Int(id),
        is_del,
    )
    .await
}

pub async fn find<'a>(
    conn: &'a sqlx::MySqlPool,
    by: &'a model::UserFindBy<'a>,
    is_del: Option<bool>,
) -> Result<Option<model::User>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, email, nickname, password, status, dateline, types, sub_exp, points, is_del FROM `user` WHERE 1=1");

    match by {
        &model::UserFindBy::ID(id) => q.push(" AND id=").push_bind(id),
        &model::UserFindBy::Email(email) => q.push(" AND email=").push_bind(email),
        &model::UserFindBy::Nickname(nickname) => q.push(" AND nickname=").push_bind(nickname),
    };

    if let Some(is_del) = is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    q.push(" LIMIT 1");

    let u = q
        .build_query_as()
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;

    Ok(u)
}

pub async fn list(
    conn: &sqlx::MySqlPool,
    with: &model::UserListWith,
) -> Result<Paginate<model::User>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, email, nickname, password, status, dateline, types, sub_exp, points, is_del FROM `user` WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM `user` WHERE 1=1");

    if let Some(email) = &with.email {
        let sql = " AND email LIKE ";
        let arg = format!("%{}%", email);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(nickname) = &with.nickname {
        let sql = " AND nickname LIKE ";
        let arg = format!("%{}%", nickname);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(status) = &with.status {
        let sql = " AND status=";

        q.push(sql).push_bind(status);
        qc.push(sql).push_bind(status);
    }

    if let Some(types) = &with.types {
        let sql = " AND types=";

        q.push(sql).push_bind(types);
        qc.push(sql).push_bind(types);
    }

    if let Some(is_del) = &with.is_del {
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

    Ok(Paginate::new(
        count.0 as u32,
        with.page,
        with.page_size,
        data,
    ))
}
