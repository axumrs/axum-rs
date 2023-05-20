use axum::{routing::post, Router};

pub fn init() -> Router {
    Router::new().route("/admin/login", post(super::admin::login))
}
