use std::sync::Arc;

use sqlx::MySqlPool;

use crate::Config;

pub struct State {
    pub pool: Arc<MySqlPool>,
    pub cfg: Arc<Config>,
}
