use std::sync::Arc;

use axum_rs::{api, config, AppState};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_toml().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                if cfg.log.is_empty() {
                    format!("{}=debug", env!("CARGO_CRATE_NAME")).into()
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
    let pool = Arc::new(pool);

    tokio::spawn(session_cleaner(pool.clone()));
    tokio::spawn(activation_cleaner(pool.clone()));
    tokio::spawn(protected_content_cleaner(pool.clone()));

    let web_addr = cfg.web.addr.as_str();

    let tcp_listener = TcpListener::bind(web_addr).await.unwrap();
    tracing::info!("Web服务监听于：{}，路由前缀：{}", web_addr, &cfg.web.prefix);

    let state = Arc::new(AppState {
        pool,
        cfg: Arc::new(cfg),
    });

    let app = api::router::init(state);

    axum::serve(tcp_listener, app).await.unwrap();
}

async fn session_cleaner(pool: Arc<PgPool>) {
    loop {
        let aff = match sqlx::query("DELETE FROM sessions WHERE expire_time <=$1")
            .bind(&chrono::Local::now())
            .execute(&*pool)
            .await
        {
            Ok(v) => v.rows_affected(),
            Err(e) => {
                tracing::error!("[session_cleaner] {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        tracing::info!("[session_cleaner] 已清理 {} 个过期会话", aff);
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

async fn activation_cleaner(pool: Arc<PgPool>) {
    loop {
        let aff = match sqlx::query("DELETE FROM activation_codes WHERE dateline <=$1")
            .bind(&(chrono::Local::now() + chrono::Duration::minutes(5)))
            .execute(&*pool)
            .await
        {
            Ok(v) => v.rows_affected(),
            Err(e) => {
                tracing::error!("[activation_cleaner] {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        tracing::info!("[activation_cleaner] 已清理 {} 个过期激活码", aff);
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn protected_content_cleaner(pool: Arc<PgPool>) {
    loop {
        let aff = match sqlx::query("DELETE FROM protected_contents WHERE expire_time <=$1")
            .bind(&chrono::Local::now())
            .execute(&*pool)
            .await
        {
            Ok(v) => v.rows_affected(),
            Err(e) => {
                tracing::error!("[protected_content_cleaner] {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        tracing::info!("[protected_content_cleaner] 已清理 {} 个过期保护内容", aff);
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
