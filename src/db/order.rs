use crate::{model, Error, Result};

use super::{pay, Paginate};

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

    // 解析交易快照
    let snap_items: Vec<model::OrderSnapItem> = match serde_json::from_str(&s.snap) {
        Ok(r) => r,
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    for item in snap_items.iter() {
        let service_type = if &item.types == "订阅" {
            model::UserPurchasedServiceType::Subscriber
        } else {
            model::UserPurchasedServiceType::Subject
        };
        if let Err(err) = sqlx::query("INSERT INTO user_purchased_service (order_id, user_id, service_id, service_type, server_num, dateline,status) VALUES(?,?,?,?,?,?,?)")
        .bind(id)
        .bind(&m.user_id)
        .bind(&item.server_id)
        .bind(&service_type)
        .bind(&item.number)
        .bind(chrono::Local::now())
        .bind(&model::UserPurchasedServiceStatus::Pending)
        .execute(&mut tx).await {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
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

pub async fn update_with_pay(
    conn: &sqlx::MySqlPool,
    id: u64,
    user_id: u32,
    pay: &model::Pay,
    pay_apply: Option<&model::PayApply>,
) -> Result<u64> {
    let mut tx = conn.begin().await.map_err(Error::from)?;

    // 已购买服务
    let purchased_services:Vec<model::UserPurchasedService> = match sqlx::query_as("SELECT id, order_id, user_id, service_id, service_type, server_num, status, dateline FROM user_purchased_service WHERE order_id=? AND user_id=?").bind(id).bind(user_id).fetch_all(&mut tx).await {
    Ok(list) => list,
    Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
   };

    let process_services = match &pay.status {
        &model::PayStatus::Finished => true,
        _ => false,
    };
    if process_services {
        for service in purchased_services.iter() {
            // 更新状态
            if let Err(err) = sqlx::query(
                "UPDATE user_purchased_service SET status=? WHERE order_id=? AND user_id=?",
            )
            .bind(model::UserPurchasedServiceStatus::Finished)
            .bind(id)
            .bind(user_id)
            .execute(&mut tx)
            .await
            {
                tx.rollback().await.map_err(Error::from)?;
                return Err(Error::from(err));
            }
            // 处理订阅
            match &service.service_type {
                &model::UserPurchasedServiceType::Subject => {}
                &model::UserPurchasedServiceType::Subscriber => {
                    let user_subscribe_info: model::UserSubscribeInfo =
                        match sqlx::query_as("SELECT types,sub_exp FROM `user` WHERE id=?")
                            .bind(user_id)
                            .fetch_one(&mut tx)
                            .await
                        {
                            Ok(i) => i,
                            Err(err) => {
                                tx.rollback().await.map_err(Error::from)?;
                                return Err(Error::from(err));
                            }
                        };
                    let now = chrono::Local::now();
                    let sub_exp_base = if (&user_subscribe_info.sub_exp).lt(&now) {
                        &now
                    } else {
                        &user_subscribe_info.sub_exp
                    };
                    let sub_exp =
                        *sub_exp_base + chrono::Duration::days(30 * service.server_num as i64);
                    if let Err(err) = sqlx::query("UPDATE `user` SET types=?, sub_exp=? WHERE id=?")
                        .bind(model::UserTypes::Subscriber)
                        .bind(sub_exp)
                        .bind(user_id)
                        .execute(&mut tx)
                        .await
                    {
                        tx.rollback().await.map_err(Error::from)?;
                        return Err(Error::from(err));
                    }
                }
            }
        }
    }

    //插入支付表
    let pay_id = match pay::add_result(&mut tx, pay).await {
        Ok(r) => r.last_insert_id(),
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    // 设置状态
    let order_status = match &pay.status {
        &model::PayStatus::Finished => model::OrderStatus::Finished,
        _ => model::OrderStatus::Pending,
    };

    // 更新订单表
    if let Err(err) = sqlx::query("UPDATE `order` SET status=?,pay_id=? WHERE id=?")
        .bind(order_status)
        .bind(pay_id)
        .bind(id)
        .execute(&mut tx)
        .await
    {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::from(err));
    }

    // 支付证明
    if let Some(pay_apply) = pay_apply {
        let pay_apply_status = match &pay.status {
            &model::PayStatus::Finished => model::PayApplyStatus::Finished,
            _ => model::PayApplyStatus::Reject,
        };
        if let Err(err) =
            sqlx::query("UPDATE pay_apply SET status=?,process_dateline=?, reason=? WHERE id=?")
                .bind(pay_apply_status)
                .bind(chrono::Local::now())
                .bind(&pay_apply.reason)
                .bind(pay_apply.id)
                .execute(&mut tx)
                .await
        {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    }

    tx.commit().await.map_err(Error::from)?;
    Ok(id)
}

pub async fn list_full_with_user(
    conn: &sqlx::MySqlPool,
    w: &model::OrderListWith,
) -> Result<Paginate<model::OrderFullWithUser>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, price, status, code, full_code, order_num, dateline, pay_id, is_del, snap, email, nickname FROM v_order_full_with_user WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM `v_order_full_with_user` WHERE 1=1");

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

pub async fn find_with_user(
    conn: &sqlx::MySqlPool,
    id: u64,
    user_id: Option<u32>,
    is_del: Option<bool>,
) -> Result<Option<model::OrderFullWithUser>> {
    let mut q = sqlx::QueryBuilder::new("SELECT id, user_id, price, status, code, full_code, order_num, dateline, pay_id, is_del, snap, email, nickname FROM v_order_full_with_user WHERE id=");
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
