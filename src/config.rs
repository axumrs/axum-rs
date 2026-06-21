use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub web_addr: String,
    pub database_url: String,
    pub database_max_conns: u32,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        ::config::Config::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize()
            .map_err(From::from)
    }

    pub fn to_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
