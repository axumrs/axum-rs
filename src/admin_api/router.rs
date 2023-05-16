use axum::{routing::get, Router};

pub fn init() -> Router {
    let subject_router = Router::new()
        .route(
            "/",
            get(super::subject::list)
                .post(super::subject::add)
                .put(super::subject::update),
        )
        .route(
            "/:id",
            get(super::subject::find)
                .delete(super::subject::del)
                .patch(super::subject::restore),
        );

    let topic_router = Router::new()
        .route("/", get(super::topic::list).post(super::topic::add))
        .route(
            "/:id",
            get(super::topic::find)
                .delete(super::topic::del)
                .patch(super::topic::restore),
        );

    let tag_router = Router::new().route("/", get(super::tag::list));

    Router::new()
        .nest("/subject", subject_router)
        .nest("/topic", topic_router)
        .nest("/tag", tag_router)
}
