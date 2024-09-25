use std::sync::Arc;

use axum_rs::{api, config, AppState};
use sqlx::postgres::PgPoolOptions;
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

    let web_addr = cfg.web.addr.as_str();

    let tcp_listener = TcpListener::bind(web_addr).await.unwrap();
    tracing::info!("Web服务监听于：{}，路由前缀：{}", web_addr, &cfg.web.prefix);

    let state = Arc::new(AppState {
        pool: Arc::new(pool),
        cfg: Arc::new(cfg),
    });

    let app = api::router::init(state);

    axum::serve(tcp_listener, app).await.unwrap();
}
