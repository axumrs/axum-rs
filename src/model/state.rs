use sqlx::MySqlPool;

pub struct State {
    pub pool: MySqlPool,
}
