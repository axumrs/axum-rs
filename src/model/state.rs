use std::sync::Arc;

use sqlx::MySqlPool;

pub struct State {
    pub pool: Arc<MySqlPool>,
}
