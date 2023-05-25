use axum::{middleware::from_extractor, routing::get, Router};

use crate::middleware::UserAuth;

pub fn init() -> Router {
    let subject_router = Router::new()
        .route("/top4", get(super::subject::top4))
        .route("/", get(super::subject::list))
        .route("/:slug", get(super::subject::detail));

    let topic_detail_router =
        Router::new().route("/:subject_slug/:slug", get(super::topic::detail));
    let topic_router = Router::new()
        .route("/top10", get(super::topic::top10))
        .route("/", get(super::topic::list))
        .nest("/", topic_detail_router);

    let tag_router = Router::new()
        .route("/", get(super::tag::list))
        .route("/:name", get(super::tag::detail));

    let user_router = Router::new()
        .route("/online_derive", get(super::user::online_derive))
        .route("/login_log", get(super::user::login_log))
        .route("/subscribe", get(super::user::subscribe))
        .route("/logout", get(super::user::logout))
        .route("/order", get(super::order::list).post(super::order::create))
        .route("/order/:id", get(super::order::find))
        .layer(from_extractor::<UserAuth>());

    Router::new()
        .nest("/subject", subject_router)
        .nest("/topic", topic_router)
        .nest("/tag", tag_router)
        .nest("/user", user_router)
}
