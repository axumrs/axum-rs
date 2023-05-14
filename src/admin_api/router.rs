use axum::{routing::post, Router};

pub fn init() -> Router {
    let subject_router = Router::new().route("/", post(super::subject::add));

    Router::new().nest("/subject", subject_router)
}
