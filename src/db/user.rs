use super::Paginate;
use crate::{model, password, Error, Result};
use chrono::{Local, TimeZone};

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

    let id = sqlx::query("INSERT INTO `user` (email, nickname, password, status, dateline, types, sub_exp, points,allow_device_num,jwt_exp) VALUES(?,?,?,?,?,?,?,?,?,?);
")
    .bind(&u.email)
    .bind(&u.nickname)
    .bind(&pwd)
    .bind(&u.status)
    .bind(&u.dateline)
    .bind(&u.types)
    .bind(&u.sub_exp)
    .bind(&u.points)
    .bind(&u.allow_device_num)
    .bind(&u.jwt_exp)
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
        .push_bind(&u.points)
        .push(", allow_device_num=")
        .push_bind(&u.allow_device_num)
        .push(", jwt_exp=")
        .push_bind(&u.jwt_exp);

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

pub async fn find<'a, T>(
    conn: T,
    by: &'a model::UserFindBy<'a>,
    is_del: Option<bool>,
) -> Result<Option<model::User>>
where
    T: sqlx::MySqlExecutor<'a>,
{
    let mut q = sqlx::QueryBuilder::new("SELECT id, email, nickname, password, status, dateline, types, sub_exp, points, is_del,allow_device_num,jwt_exp FROM `user` WHERE 1=1");

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
    let mut q = sqlx::QueryBuilder::new("SELECT id, email, nickname, password, status, dateline, types, sub_exp, points, is_del,allow_device_num,jwt_exp FROM `user` WHERE 1=1");
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

pub async fn login<'a>(
    conn: &'a sqlx::MySqlPool,
    meta: &'a model::UserLoginMeta,
) -> Result<(model::User, u64)> {
    let mut tx = conn.begin().await.map_err(Error::from)?;

    let u = find(&mut tx, &model::UserFindBy::Email(&meta.email), Some(false)).await?;
    if u.is_none() {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::not_found("用户名或密码错误"));
    }

    let mut u = u.unwrap();
    if !password::verify(&meta.password, &u.password)? {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::not_found("用户名或密码错误"));
    }

    match &u.status {
        &model::UserStatus::Actived => {
            // pass
        }
        &model::UserStatus::Freezed => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::not_found("你的账号已被冻结"));
        }
        &model::UserStatus::Pending => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::not_found("你的账号尚未激活"));
        }
    };

    // 登录日志
    let id = match sqlx::query("INSERT INTO user_login_log (user_id, ip, browser, os, device, dateline, is_del) VALUES(?,?,?,?,?,?,?)")
    .bind(&u.id)
    .bind(&meta.ip)
    .bind(&meta.uai.browser)
    .bind(&meta.uai.os)
    .bind(&meta.uai.device)
    .bind(chrono::Local::now())
    .bind(false)
    .execute(&mut tx).await {
        Ok(r) => r.last_insert_id(),
        Err(err) => {
             tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    if let Err(err) =
        sqlx::query("INSERT INTO user_login_log_agent (log_id, user_agent) VALUES(?,?)")
            .bind(id)
            .bind(&meta.ua)
            .execute(&mut tx)
            .await
    {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::from(err));
    }

    // 如果已过期

    match &u.types {
        &model::UserTypes::Normal => {
            // pass
        }
        &model::UserTypes::Subscriber => {
            if (&u.sub_exp).lt(&chrono::Local::now()) {
                if let Err(err) =
                    sqlx::query("UPDATE `user` SET types=?,allow_device_num=1,jwt_exp=0 WHERE id=?")
                        .bind(model::UserTypes::Normal)
                        .bind(u.id)
                        .execute(&mut tx)
                        .await
                {
                    tx.rollback().await.map_err(Error::from)?;
                    return Err(Error::from(err));
                }
                u = model::User {
                    types: model::UserTypes::Normal,
                    allow_device_num: 1,
                    jwt_exp: 0,
                    ..u
                };
            }
        }
    }

    tx.commit().await.map_err(Error::from)?;

    Ok((u, id))
}

pub async fn basic_info(conn: &sqlx::MySqlPool, id: u32) -> Result<model::UserBasicInfo> {
    let u = sqlx::query_as("SELECT id, email, nickname, dateline, types, sub_exp, points,allow_device_num,jwt_exp FROM `user` WHERE is_del=? AND status=? AND id=? LIMIT 1")
        .bind(&false)
        .bind(&model::UserStatus::Actived)
        .bind(&id)
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    Ok(u)
}

pub async fn check_in(conn: &sqlx::MySqlPool, id: u32, points: u32) -> Result<u64> {
    let mut tx = conn.begin().await.map_err(Error::from)?;
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format("%Y-%m-%d").to_string();

    let start = format!("{} 00:00:00", now);
    let start = Local.datetime_from_str(&start, fmt).map_err(Error::from)?;

    let end = format!("{} 23:59:59", now);
    let end = Local.datetime_from_str(&end, fmt).map_err(Error::from)?;

    let count: (i64,) = match sqlx::query_as(
        "SELECT COUNT(*) FROM user_check_in WHERE user_id=? AND (dateline BETWEEN ? AND ?)",
    )
    .bind(id)
    .bind(&start)
    .bind(&end)
    .fetch_one(&mut tx)
    .await
    {
        Ok(r) => r,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    if count.0 > 0 {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::already_exists("你今天已经签过到了"));
    }

    let id = match sqlx::query("INSERT INTO user_check_in(user_id,points,dateline) VALUES(?,?,?)")
        .bind(id)
        .bind(points)
        .bind(&now)
        .execute(&mut tx)
        .await
    {
        Ok(r) => r.last_insert_id(),
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    if let Err(err) = sqlx::query("UPDATE `user` SET points=points+? WHERE id=?")
        .bind(points)
        .bind(id)
        .execute(&mut tx)
        .await
    {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::from(err));
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(id)
}

fn check_pwd(u: &model::User, pwd: &str) -> Result<bool> {
    password::verify(pwd, &u.password)
}

pub async fn change_pwd(conn: &sqlx::MySqlPool, pwd: &str, new_pwd: &str, id: u32) -> Result<u64> {
    let u = find(conn, &model::UserFindBy::ID(id), Some(false)).await?;
    if u.is_none() {
        return Err(Error::not_found("不存在的用户"));
    }

    let u = u.unwrap();

    if !check_pwd(&u, pwd)? {
        return Err(Error::not_found("密码错误"));
    }

    let hashed_new_pwd = password::hash(new_pwd)?;

    let aff = sqlx::query("UPDATE `user` SET password=? WHERE id=?")
        .bind(&hashed_new_pwd)
        .bind(id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn update_profile(conn: &sqlx::MySqlPool, up: &model::User2Profile) -> Result<u64> {
    let u = find(conn, &model::UserFindBy::ID(up.id), Some(false)).await?;
    if u.is_none() {
        return Err(Error::not_found("不存在的用户"));
    }

    let u = u.unwrap();

    if !check_pwd(&u, &up.password)? {
        return Err(Error::not_found("密码错误"));
    }

    let mut tx = conn.begin().await.map_err(Error::from)?;
    let email_count: (i64,) =
        match sqlx::query_as("SELECT COUNT(*) FROM `user` WHERE email=? AND id<>?")
            .bind(&up.email)
            .bind(&up.id)
            .fetch_one(&mut tx)
            .await
        {
            Ok(r) => r,
            Err(err) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(Error::from(err));
            }
        };

    let nickname_count: (i64,) =
        match sqlx::query_as("SELECT COUNT(*) FROM `user` WHERE nickname=? AND id<>?")
            .bind(&up.nickname)
            .bind(&up.id)
            .fetch_one(&mut tx)
            .await
        {
            Ok(r) => r,
            Err(err) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(Error::from(err));
            }
        };

    if email_count.0 > 0 {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::already_exists("email已存在"));
    }

    if nickname_count.0 > 0 {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::already_exists("昵称已存在"));
    }

    // email, nickname, password, status, dateline, types, sub_exp, points, is_del,allow_device_num,jwt_exp
    let q = match &u.types {
        &model::UserTypes::Normal => sqlx::query("UPDATE `user` SET email=?,nickname=? WHERE id=?")
            .bind(&up.email)
            .bind(&up.nickname)
            .bind(&up.id),
        &model::UserTypes::Subscriber => sqlx::query(
            "UPDATE `user` SET email=?,nickname=?,allow_device_num=?,jwt_exp=? WHERE id=?",
        )
        .bind(&up.email)
        .bind(&up.nickname)
        .bind(&up.allow_device_num)
        .bind(&up.jwt_exp)
        .bind(&up.id),
    };

    let aff = match q.execute(&mut tx).await {
        Ok(r) => r.rows_affected(),
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    tx.commit().await.map_err(Error::from)?;
    Ok(aff)
}

pub async fn profile(conn: &sqlx::MySqlPool, id: u32) -> Result<Option<model::User2Profile>> {
    let up = sqlx::query_as("SELECT id, email, nickname, password, allow_device_num, jwt_exp FROM `user` WHERE is_del=? AND status=? AND id=? LIMIT 1").bind(false).bind(&model::UserStatus::Actived).bind(id).fetch_optional(conn).await.map_err(Error::from)?;
    Ok(up)
}
