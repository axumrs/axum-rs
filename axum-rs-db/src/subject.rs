use axum_rs_model::{pagination, subject as model};

use sqlx::{PgExecutor, PgTransaction, Postgres, QueryBuilder, Result, query, query_as};

pub mod filter {
    use axum_rs_model::pagination;

    pub enum FindBy<'a> {
        Id(&'a uuid::Uuid),
        Slug(&'a str),
    }
    pub struct FindFilter<'a> {
        pub by: FindBy<'a>,
        pub is_del: Option<bool>,
    }

    pub enum ListOrderBy {
        IdDesc,
        PinDesc,
    }

    pub struct ListFilter {
        pub pq: pagination::PaginationQuery,
        pub is_del: Option<bool>,
        pub name: Option<String>,
        pub slug: Option<String>,
        pub price: Option<(i32, i32)>,
        pub order: Option<ListOrderBy>,
    }
}

pub async fn create(e: impl PgExecutor<'_>, m: model::Subject) -> Result<model::Subject> {
    let mut q = QueryBuilder::new(
        r#"INSERT INTO "subjects" ("id", "name", "slug", "summary", "is_del", "cover", "status", "price", "pin") "#,
    );
    q.push_values(&[&m], |mut b, m| {
        b.push_bind(&m.id)
            .push_bind(&m.name)
            .push_bind(&m.slug)
            .push_bind(&m.summary)
            .push_bind(&m.is_del)
            .push_bind(&m.cover)
            .push_bind(&m.status)
            .push_bind(&m.price)
            .push_bind(&m.pin);
    });
    q.push(" RETURNING *");

    q.build_query_as().fetch_one(e).await
}

pub async fn update(e: impl PgExecutor<'_>, m: model::Subject) -> Result<model::Subject> {
    let sql = r#"UPDATE "subjects" SET "name" = $2, "slug" = $3, "summary" = $4, "is_del" = $5, "cover" = $6, "status" = $7, "price" = $8, "pin" = $9 WHERE "id" = $1 RETURNING *"#;
    query_as(sql)
        .bind(&m.id)
        .bind(&m.name)
        .bind(&m.slug)
        .bind(&m.summary)
        .bind(&m.is_del)
        .bind(&m.cover)
        .bind(&m.status)
        .bind(&m.price)
        .bind(&m.pin)
        .fetch_one(e)
        .await
}

pub async fn delete_or_recover(
    e: impl PgExecutor<'_>,
    id: &uuid::Uuid,
    is_del: bool,
) -> Result<u64> {
    let sql = r#"UPDATE "subjects" SET "is_del" = $2 WHERE "id" = $1"#;
    let aff = query(sql)
        .bind(&id)
        .bind(&is_del)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

pub async fn real_delete(e: impl PgExecutor<'_>, id: &uuid::Uuid) -> Result<u64> {
    let sql = r#"DELETE FROM "subjects" WHERE "id" = $1"#;
    let aff = query(sql).bind(&id).execute(e).await?.rows_affected();
    Ok(aff)
}

pub async fn find<'a>(
    e: impl PgExecutor<'a>,
    f: &filter::FindFilter<'a>,
) -> Result<Option<model::Subject>> {
    let mut q = QueryBuilder::new(
        r#"SELECT "id", "name", "slug", "summary", "is_del", "cover", "status", "price", "pin" FROM "subjects" WHERE 1=1"#,
    );
    if let Some(v) = &f.is_del {
        q.push(r#" AND "is_del"= "#).push_bind(v);
    }
    match f.by {
        filter::FindBy::Id(id) => {
            q.push(r#" AND "id"= "#).push_bind(id);
        }
        filter::FindBy::Slug(slug) => {
            q.push(r#" AND "slug"= "#).push_bind(slug);
        }
    };
    q.push(" LIMIT 1");
    q.build_query_as().fetch_optional(e).await
}

pub async fn list<'a>(
    e: impl PgExecutor<'a>,
    f: &'a filter::ListFilter,
) -> Result<Vec<model::Subject>> {
    let q = QueryBuilder::new(
        r#"SELECT "id", "name", "slug", "summary", "is_del", "cover", "status", "price", "pin" FROM "subjects" WHERE 1=1"#,
    );
    let mut q = build_list_query(q, f);
    let order = match &f.order {
        Some(v) => match v {
            filter::ListOrderBy::IdDesc => r#" "id" DESC "#,
            filter::ListOrderBy::PinDesc => r#" "pin" DESC "#,
        },
        None => r#" "id" DESC "#,
    };
    q.push(" ORDER BY ")
        .push(order)
        .push(" LIMIT ")
        .push_bind(f.pq.page_size())
        .push(" OFFSET ")
        .push_bind(f.pq.limit());
    q.build_query_as().fetch_all(e).await
}

pub async fn list_count<'a>(e: impl PgExecutor<'a>, f: &'a filter::ListFilter) -> Result<i64> {
    let q = QueryBuilder::new(r#"SELECT COUNT(*) FROM "subjects" WHERE 1=1"#);
    let mut q = build_list_query(q, f);
    q.build_query_scalar().fetch_one(e).await
}

pub async fn pagination_list<'a>(
    tx: &'a mut PgTransaction<'a>,
    f: &'a filter::ListFilter,
) -> Result<pagination::Pagination<model::Subject>> {
    let data = list(&mut **tx, f).await?;
    let total = list_count(&mut **tx, f).await?;
    Ok(pagination::Pagination::quick(&f.pq, total, data))
}

fn build_list_query<'a>(
    mut q: QueryBuilder<'a, Postgres>,
    f: &'a filter::ListFilter,
) -> QueryBuilder<'a, Postgres> {
    if let Some(v) = f.is_del {
        q.push(r#" AND "is_del" = "#).push_bind(v);
    }
    if let Some(v) = &f.name {
        q.push(r#" AND "name" ILIKE "#).push_bind(format!("%{v}%"));
    }
    if let Some(v) = &f.slug {
        q.push(r#" AND "slug" ILIKE "#).push_bind(format!("%{v}%"));
    }
    if let Some((min_price, max_price)) = &f.price {
        q.push(r#" AND ("price" BETWEEN "#)
            .push_bind(min_price)
            .push(r#" AND "#)
            .push_bind(max_price)
            .push(")");
    }
    q
}
