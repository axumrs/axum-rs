use std::sync::Arc;

use axum::{Extension, Router};
use axum_rs::{model::State, Config};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = Config::from_env()
        .map_err(|e| tracing::error!("初始化配置失败：{}", e.to_string()))
        .unwrap();
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(cfg.mysql.max_conns)
        .connect(&cfg.mysql.dsn)
        .await
        .map_err(|e| tracing::error!("初始化数据库失败：{}", e.to_string()))
        .unwrap();

    tracing::info!("Web服务监听于{}", &cfg.web.addr);

    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .layer(Extension(Arc::new(State { pool })));

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
