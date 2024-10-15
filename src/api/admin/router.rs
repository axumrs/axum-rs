use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::{mid, ArcAppState};

use super::{subject, tag, topic};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/subject", subject_init(state.clone()))
        .nest("/tag", tag_init(state.clone()))
        .nest("/topic", topic_init(state.clone()))
        .layer(middleware::from_extractor_with_state::<
            mid::AdminAuth,
            ArcAppState,
        >(state.clone()))
}

fn subject_init(state: ArcAppState) -> Router {
    Router::new()
        .route(
            "/",
            get(subject::list).post(subject::add).put(subject::edit),
        )
        .route("/:id", delete(subject::del).patch(subject::res))
        .route("/all", get(subject::all))
        .with_state(state)
}

fn tag_init(state: ArcAppState) -> Router {
    Router::new().route("/all", get(tag::all)).with_state(state)
}

fn topic_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", post(topic::add).put(topic::edit).get(topic::list))
        .with_state(state)
}
