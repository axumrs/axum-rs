use axum::{Router, routing::get};
use axum_rs::{AppState, Asset, Config, handler, log};

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .thread_name(env!("CARGO_CRATE_NAME"))
        .enable_all()
        .build()?;
    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    log::init();
    let cfg = Config::from_env()?.to_arc();
    let web_addr = cfg.web_addr.clone();
    let listener = tokio::net::TcpListener::bind(&web_addr).await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.database_max_conns)
        .connect(&cfg.database_url)
        .await?;
    let state = AppState::new_arc(cfg, pool);

    tracing::info!("listening on {}", web_addr);
    let app = Router::new()
        .route("/", get(handler::index))
        .fallback(Asset::static_handler)
        .with_state(state);
    axum::serve(listener, app).await?;
    Ok(())
}
