use axum_rs::{config::Config, utils};
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

    let tts = find_non_tag_topics(&pool).await?;

    if let Err(e) = update_tags(&pool, tts).await {
        tracing::error!("更新标签失败：{e}");
        return Err(e);
    }

    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TopicWithTags {
    pub topic_id: String,
    pub subject_id: String,
    pub title: String,
    pub tag_ids: Vec<String>,
}
async fn find_non_tag_topics(pool: &sqlx::PgPool) -> anyhow::Result<Vec<TopicWithTags>> {
    sqlx::query_as(
        r#"WITH 
	first_topic_tag AS (
		SELECT * FROM (
		SELECT *, row_number() over (partition by subject_id) as rn FROM
			(SELECT id, subject_id FROM topics WHERE is_del=false ORDER BY id ASC) AS t
			INNER JOIN
			(SELECT topic_id, array_agg(tag_id) as tag_ids FROM topic_tags group by topic_id HAVING array_agg(tag_id) is not null ORDER BY topic_id asc) AS tt
			ON t.id=tt.topic_id
			)
		WHERE rn =1
	
	),
	subject_topic_ids AS (
		SELECT ARRAY_AGG(id) AS topic_ids, subject_id FROM topics WHERE is_del=false GROUP BY subject_id
	),
	non_tag_topics AS (
		SELECT id AS topic_id FROM topics WHERE is_del=false
		EXCEPT ALL
		SELECT topic_id FROM topic_tags
	)
	SELECT * FROM 
		(
			SELECT id as topic_id,subject_id,title FROM non_tag_topics AS ntt
			INNER JOIN topics AS t
			ON ntt.topic_id=t.id
		) AS t0
	INNER JOIN
	(
		SELECT tag_ids, subject_id
		FROM 
			first_topic_tag AS ftt
		INNER JOIN
			subject_topic_ids AS sti
		USING (subject_id)
	) AS t1
	USING(subject_id)
		"#,
    ).fetch_all(pool).await.map_err(|e|e.into())
}

async fn update_tags(pool: &sqlx::PgPool, tts: Vec<TopicWithTags>) -> anyhow::Result<u64> {
    let mut data = vec![];

    for tt in tts {
        for t in tt.tag_ids {
            data.push((tt.topic_id.clone(), t));
        }
    }
    let mut q = sqlx::QueryBuilder::new("INSERT INTO topic_tags (id,topic_id,tag_id) ");

    q.push_values(&data, |mut b, m| {
        b.push_bind(utils::id::new())
            .push_bind(&m.0)
            .push_bind(&m.1);
    });

    let aff = q.build().execute(pool).await?.rows_affected();
    Ok(aff)
}

async fn back_table(pool: &sqlx::PgPool) -> anyhow::Result<u64> {
    let now = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let table_name = format!("topic_tags_{}", now);
    tracing::info!("开始备份数据表：{}", table_name);

    let sql = format!("CREATE TABLE {} AS SELECT * FROM topic_tags", table_name);
    let aff = sqlx::query(&sql).execute(pool).await?.rows_affected();

    tracing::info!("数据表备份完成");
    Ok(aff)
}
