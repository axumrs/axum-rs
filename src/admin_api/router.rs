use axum::{routing::get, Router};

pub fn init() -> Router {
    let subject_router =
        Router::new().route("/", get(super::subject::list).post(super::subject::add));

    Router::new().nest("/subject", subject_router)
}
