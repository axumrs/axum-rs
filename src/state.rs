use std::sync::Arc;

use sqlx::PgPool;

use crate::Config;

pub struct AppState {
    pub cfg: Arc<Config>,
    pub pool: PgPool,
}

impl AppState {
    pub fn new_arc(cfg: Arc<Config>, pool: PgPool) -> Arc<Self> {
        Arc::new(AppState { cfg, pool })
    }
}

pub type ArcAppState = Arc<AppState>;
