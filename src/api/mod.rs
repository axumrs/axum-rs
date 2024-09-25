use std::sync::Arc;

use crate::{AppState, Error};

pub(super) mod admin;
pub(super) mod auth;
pub mod router;
pub(super) mod user;
pub(super) mod web;

fn log_error(handler_name: &str) -> Box<dyn Fn(Error) -> Error> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("👉 [{}] - {:?}", handler_name, err);
        err
    })
}

fn get_pool(state: &AppState) -> Arc<sqlx::PgPool> {
    state.pool.clone()
}
