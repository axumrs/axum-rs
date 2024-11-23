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

    let mut ses_handler = tokio::spawn(session_cleaner(pool.clone(), cfg.cleaner_max_try));
    let mut act_handler = tokio::spawn(activation_cleaner(pool.clone(), cfg.cleaner_max_try));
    let mut pc_handler = tokio::spawn(protected_content_cleaner(pool.clone(), cfg.cleaner_max_try));

    let web_addr = cfg.web.addr.as_str();

    let tcp_listener = TcpListener::bind(web_addr).await.unwrap();
    tracing::info!("Web服务监听于：{}，路由前缀：{}", web_addr, &cfg.web.prefix);

    let state = Arc::new(AppState {
        pool,
        cfg: Arc::new(cfg),
    });

    let app = api::router::init(state);

    let mut svr_handler = tokio::spawn(async move { axum::serve(tcp_listener, app).await });

    loop {
        tokio::select! {
            _ = &mut svr_handler => {
                tracing::info!("Web服务退出");
                ses_handler.abort();
                act_handler.abort();
                pc_handler.abort();
                break;
            }
            _ = &mut ses_handler => {
                tracing::info!("会话清理退出");
                act_handler.abort();
                pc_handler.abort();
                svr_handler.abort();
                break;
            }
            _ = &mut act_handler => {
                tracing::info!("激活码清理退出");
                ses_handler.abort();
                pc_handler.abort();
                svr_handler.abort();
                break;
            }
            _ = &mut pc_handler => {
                tracing::info!("内容保护清理退出");
                ses_handler.abort();
                act_handler.abort();
                svr_handler.abort();
                break;
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Ctrl+C退出");
                ses_handler.abort();
                act_handler.abort();
                pc_handler.abort();
                svr_handler.abort();
                break;
            }
        }
    }
}

async fn session_cleaner(pool: Arc<PgPool>, max_try: u32) {
    let mut tried = 0u32;
    loop {
        if max_try > 0 && tried >= max_try {
            tracing::info!("[session_cleaner] 已尝试 {} 次", tried);
            break;
        }
        let aff = match sqlx::query("DELETE FROM sessions WHERE expire_time <=$1")
            .bind(&chrono::Local::now())
            .execute(&*pool)
            .await
        {
            Ok(v) => {
                tried = 0;
                v.rows_affected()
            }
            Err(e) => {
                tried += 1;
                tracing::error!("[session_cleaner] {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        tracing::info!("[session_cleaner] 已清理 {} 个过期会话", aff);
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

async fn activation_cleaner(pool: Arc<PgPool>, max_try: u32) {
    let mut tried = 0u32;
    loop {
        if max_try > 0 && tried >= max_try {
            tracing::info!("[activation_cleaner] 已尝试 {} 次", tried);
            break;
        }
        let aff = match sqlx::query("DELETE FROM activation_codes WHERE expire_time <=$1")
            .bind(&(chrono::Local::now()))
            .execute(&*pool)
            .await
        {
            Ok(v) => {
                tried = 0;
                v.rows_affected()
            }
            Err(e) => {
                tried += 1;
                tracing::error!("[activation_cleaner] {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        tracing::info!("[activation_cleaner] 已清理 {} 个过期激活码", aff);
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

async fn protected_content_cleaner(pool: Arc<PgPool>, max_try: u32) {
    let mut tried = 0u32;
    loop {
        if max_try > 0 && tried >= max_try {
            tracing::info!("[protected_content_cleaner] 已尝试 {} 次", tried);
            break;
        }

        let aff = match sqlx::query("DELETE FROM protected_contents WHERE expire_time <=$1")
            .bind(&chrono::Local::now())
            .execute(&*pool)
            .await
        {
            Ok(v) => {
                tried = 0;
                v.rows_affected()
            }
            Err(e) => {
                tried += 1;
                tracing::error!("[protected_content_cleaner] {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5000)).await;
                continue;
            }
        };
        tracing::info!("[protected_content_cleaner] 已清理 {} 个过期保护内容", aff);
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
