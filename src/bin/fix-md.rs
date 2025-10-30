use axum_rs::{config::Config, model, service, utils};

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("使用方法：{} <yes|no>", args[0]);
        return Ok(());
    }
    if args[1].to_lowercase() != "yes" {
        eprintln!("未确认执行，退出");
        return Ok(());
    }

    let cfg = Config::from_toml().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                if cfg.log.is_empty() {
                    format!("{}=debug", env!("CARGO_BIN_NAME")).into()
                } else {
                    cfg.log.as_str().into()
                }
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(cfg.db.max_conns)
        .connect(&cfg.db.dsn)
        .await
        .unwrap();

    if let Err(e) = back_table(&pool).await {
        tracing::error!("备份数据表失败：{e}");
        return Err(e);
    }

    if let Err(e) = fix_markdown(&pool, &cfg.topic_section_secret_key).await {
        tracing::error!("修复 markdown 失败：{e}");
        return Err(e);
    }
    Ok(())
}

async fn fix_markdown(pool: &sqlx::PgPool, hash_secret_key: &str) -> anyhow::Result<()> {
    let topic_list = model::topic::Topic::list_all(
        pool,
        &model::topic::TopicListAllFilter {
            limit: None,
            order: Some("id ASC".into()),
            is_del: Some(false),
        },
    )
    .await?;

    for topic in &topic_list {
        let mut tx = pool.begin().await?;
        // 清空段落
        if let Err(e) = service::topic_section::clean(&mut *tx, &topic.id).await {
            tx.rollback().await?;
            tracing::error!("清空段落失败:{} - {}", topic.id, e);
            continue;
        }
        // 生成段落
        let sects = match utils::topic::sections(topic, hash_secret_key) {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await?;
                tracing::error!("生成段落失败:{} - {}", topic.id, e);
                continue;
            }
        };
        // 段落入库
        let mut q = sqlx::QueryBuilder::new(
            r#"INSERT INTO "topic_sections" ("id", "topic_id", "sort", "hash", "content") "#,
        );
        q.push_values(&sects, |mut b, s| {
            b.push_bind(&s.id)
                .push_bind(&s.topic_id)
                .push_bind(&s.sort)
                .push_bind(&s.hash)
                .push_bind(&s.content);
        });

        if let Err(e) = q.build().execute(&mut *tx).await {
            tx.rollback().await?;
            tracing::error!("段落入库失败:{} - {}", topic.id, e);
            continue;
        }

        tx.commit().await?;
    }

    Ok(())
}

async fn back_table(pool: &sqlx::PgPool) -> anyhow::Result<u64> {
    let now = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let table_name = format!("topic_sections_{}", now);
    tracing::info!("开始备份数据表：{}", table_name);

    let sql = format!(
        "CREATE TABLE {} AS SELECT * FROM topic_sections",
        table_name
    );
    let aff = sqlx::query(&sql).execute(pool).await?.rows_affected();

    tracing::info!("数据表备份完成");
    Ok(aff)
}
