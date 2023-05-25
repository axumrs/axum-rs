use crate::{model, Error, Result};

use super::Paginate;

pub async fn add(conn: &sqlx::MySqlPool, m: &model::Order, s: &model::OrderSnap) -> Result<u64> {
    let mut tx = conn.begin().await.map_err(Error::from)?;

    let id = match sqlx::query("INSERT INTO `order` (user_id, price, status, code, full_code, order_num, dateline, pay_id, is_del) VALUES(?,?,?,?,?,?,?,?,?)")
    .bind(&m.user_id)
    .bind(&m.price)
    .bind(&m.status)
    .bind(&m.code)
    .bind(&m.full_code)
    .bind(&m.order_num)
    .bind(&m.dateline)
    .bind(&m.pay_id)
    .bind(&m.is_del)
    .execute(&mut tx).await {
        Ok(r) => r.last_insert_id(),
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    if let Err(err) = sqlx::query("INSERT INTO order_snap (order_id, snap) VALUES(?,?)")
        .bind(&id)
        .bind(&s.snap)
        .execute(&mut tx)
        .await
    {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::from(err));
    }

    tx.commit().await.map_err(Error::from)?;

    Ok(id)
}

pub async fn find(
    conn: &sqlx::MySqlPool,
    id: u64,
    user_id: Option<u32>,
    is_del: Option<bool>,
) -> Result<Option<model::OrderFull>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, price, status, code, full_code, order_num, dateline, pay_id, is_del, snap FROM v_order_full WHERE id=");
    q.push_bind(id);

    if let Some(user_id) = user_id {
        q.push(" AND user_id=").push_bind(user_id);
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

pub async fn list(
    conn: &sqlx::MySqlPool,
    w: &model::OrderListWith,
) -> Result<Paginate<model::OrderWithUser>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, price, status, code, full_code, order_num, dateline, pay_id, is_del, email, nickname FROM v_order_with_user WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM `v_order_with_user` WHERE 1=1");

    if let Some(email) = &w.email {
        let sql = " AND email LIKE ";
        let arg = format!("%{}%", email);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(nickname) = &w.nickname {
        let sql = " AND nickname LIKE ";
        let arg = format!("%{}%", nickname);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(pay_id) = &w.pay_id {
        let sql = " AND pay_id=";

        q.push(sql).push_bind(pay_id);
        qc.push(sql).push_bind(pay_id);
    }

    if let Some(code) = &w.code {
        let sql = " AND code LIKE ";
        let arg = format!("%{}%", code);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(order_num) = &w.order_num {
        let sql = " AND order_num LIKE ";
        let arg = format!("%{}%", order_num);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(status) = &w.status {
        let sql = " AND status=";

        q.push(sql).push_bind(status);
        qc.push(sql).push_bind(status);
    }

    if let Some(is_del) = &w.is_del {
        let sql = " AND is_del=";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    if let Some(user_id) = &w.user_id {
        let sql = " AND user_id=";

        q.push(sql).push_bind(user_id);
        qc.push(sql).push_bind(user_id);
    }

    q.push(" ORDER BY id DESC ")
        .push(" LIMIT ")
        .push_bind(w.page_size)
        .push(" OFFSET ")
        .push_bind(w.page * w.page_size);

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

    Ok(Paginate::new(count.0 as u32, w.page, w.page_size, data))
}
