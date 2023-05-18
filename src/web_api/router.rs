use axum::{routing::get, Router};

pub fn init() -> Router {
    let subject_router = Router::new()
        .route("/top4", get(super::subject::top4))
        .route("/", get(super::subject::list))
        .route("/:slug", get(super::subject::detail));

    let topic_router = Router::new()
        .route("/top10", get(super::topic::top10))
        .route("/", get(super::topic::list));

    let tag_router = Router::new().route("/", get(super::tag::list));

    Router::new()
        .nest("/subject", subject_router)
        .nest("/topic", topic_router)
        .nest("/tag", tag_router)
}
