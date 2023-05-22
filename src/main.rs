use std::sync::Arc;

use axum::{Extension, Router};
use axum_rs::{admin_api, auth_api, model::State, web_api, Config};
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
    let rds = redis::Client::open(cfg.redis.dsn.as_str()).unwrap();

    let web_addr = &cfg.web.addr.clone();
    tracing::info!("Web服务监听于{}", web_addr);

    let web_router = web_api::router::init();
    let admin_router = admin_api::router::init();
    let auth_router = auth_api::router::init();

    let app = Router::new()
        .nest("/web", web_router)
        .nest("/admin", admin_router)
        .nest("/auth", auth_router)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .layer(Extension(Arc::new(State {
            pool: Arc::new(pool),
            cfg: Arc::new(cfg),
            rds: Arc::new(rds),
        })));

    axum::Server::bind(&web_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
