use axum::{
    routing::{delete, get},
    Router,
};

pub fn init() -> Router {
    let subject_router = Router::new()
        .route("/", get(super::subject::list).post(super::subject::add))
        .route(
            "/:id",
            delete(super::subject::del).patch(super::subject::restore),
        );

    Router::new().nest("/subject", subject_router)
}
