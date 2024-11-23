use std::sync::Arc;

use sqlx::PgPool;

use crate::config::Config;

pub struct AppState {
    pub pool: Arc<PgPool>,
    pub cfg: Arc<Config>,
}

pub type ArcAppState = Arc<AppState>;
