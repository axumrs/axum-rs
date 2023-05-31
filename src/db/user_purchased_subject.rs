use crate::{model, Error, Result};

use super::Paginate;

pub async fn find<'a>(
    conn: &'a sqlx::MySqlPool,
    by: &'a model::UserPurchasedSubjectFindBy<'a>,
    is_del: Option<bool>,
) -> Result<Option<model::UserPurchasedSubject>> {
    let mut q = sqlx::QueryBuilder::new("SELECT purchased_id, order_id, user_id, service_id, service_type, server_num, purchased_status, purchased_dateline, email, nickname, id, slug, name, summary, cover, status, price, is_del FROM v_user_purchased_subject WHERE 1=1");

    match by {
        &model::UserPurchasedSubjectFindBy::PurchasedID(id) => {
            q.push(" AND purchased_id=").push_bind(id)
        }
        &model::UserPurchasedSubjectFindBy::PurchasedIDWithUser { id, user_id } => q
            .push(" AND purchased_id=")
            .push_bind(id)
            .push(" AND user_id=")
            .push_bind(user_id),
        &model::UserPurchasedSubjectFindBy::SubjectID {
            subject_id,
            user_id,
        } => q
            .push(" AND id=")
            .push_bind(subject_id)
            .push(" AND user_id=")
            .push_bind(user_id),
        &model::UserPurchasedSubjectFindBy::Subject { slug, user_id } => q
            .push(" AND slug=")
            .push_bind(slug)
            .push(" AND user_id=")
            .push_bind(user_id),
    };

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
    with: &model::UserPurchasedSubjectListWith,
) -> Result<Paginate<model::UserPurchasedSubject>> {
    let mut q = sqlx::QueryBuilder::new("SELECT purchased_id, order_id, user_id, service_id, service_type, server_num, purchased_status, purchased_dateline, email, nickname, id, slug, name, summary, cover, status, price, is_del FROM v_user_purchased_subject WHERE 1=1");
    let mut qc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM v_user_purchased_subject WHERE 1=1");

    if let Some(user_id) = &with.user_id {
        let sql = " AND user_id=";

        q.push(sql).push_bind(user_id);
        qc.push(sql).push_bind(user_id);
    }

    if let Some(subject_id) = &with.subject_id {
        let sql = " AND id=";

        q.push(sql).push_bind(subject_id);
        qc.push(sql).push_bind(subject_id);
    }

    if let Some(subject_slug) = &with.subject_slug {
        let sql = " AND slug LIKE ";
        let arg = format!("%{}%", subject_slug);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

    if let Some(purchased_id) = &with.purchased_id {
        let sql = " AND purchased_id=";

        q.push(sql).push_bind(purchased_id);
        qc.push(sql).push_bind(purchased_id);
    }

    if let Some(subject_name) = &with.subject_name {
        let sql = " AND name LIKE ";
        let arg = format!("%{}%", subject_name);

        q.push(sql).push_bind(arg.clone());
        qc.push(sql).push_bind(arg);
    }

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

    if let Some(is_del) = &with.is_del {
        let sql = " AND is_del=";

        q.push(sql).push_bind(is_del);
        qc.push(sql).push_bind(is_del);
    }

    q.push(" ORDER BY purchased_id DESC")
        .push(" LIMIT ")
        .push_bind(with.pw.page_size)
        .push(" OFFSET ")
        .push_bind(with.pw.offset());

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

    Ok(Paginate::with(&count, &with.pw, data))
}

pub async fn is_purchased(
    conn: &sqlx::MySqlPool,
    subject_slug: &str,
    user_id: u32,
) -> Result<bool> {
    let p = find_by_subject(conn, subject_slug, user_id).await?;

    Ok(p.is_some())
}
pub async fn find_by_subject(
    conn: &sqlx::MySqlPool,
    subject_slug: &str,
    user_id: u32,
) -> Result<Option<model::UserPurchasedSubject>> {
    find(
        conn,
        &model::UserPurchasedSubjectFindBy::Subject {
            slug: subject_slug,
            user_id,
        },
        Some(false),
    )
    .await
}

pub async fn select_in(
    conn: &sqlx::MySqlPool,
    subject_ids: &[u32],
    user_id: u32,
    is_del: Option<bool>,
) -> Result<Vec<model::UserPurchasedSubject>> {
    let mut q = sqlx::QueryBuilder::new("SELECT purchased_id, order_id, user_id, service_id, service_type, server_num, purchased_status, purchased_dateline, email, nickname, id, slug, name, summary, cover, status, price, is_del FROM v_user_purchased_subject WHERE 1=1");

    //let subject_str_ids: Vec<String> = subject_ids.into_iter().map(|id| id.to_string()).collect();
    //let subject_str_ids: String = subject_str_ids.join(",");
    q.push(" AND (id) IN ")
        .push_tuples(subject_ids.iter(), |mut b, id| {
            b.push_bind(id);
        })
        //.push_bind(&subject_str_ids)
        //.push(")")
        .push(" AND user_id=")
        .push_bind(user_id);

    if let Some(is_del) = is_del {
        q.push(" AND is_del=").push_bind(is_del);
    }

    q.push(" ORDER BY id DESC");

    let r = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(r)
}
