use std::sync::Arc;

use crate::{model::State, Error};

pub fn log_error(handler_name: &str) -> Box<dyn Fn(Error) -> Error> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("👉 [{}] - {:?}", handler_name, err);
        err
    })
}

pub fn get_conn(state: &State) -> Arc<sqlx::MySqlPool> {
    state.pool.clone()
}
