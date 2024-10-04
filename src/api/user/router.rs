use axum::{middleware, routing::get, Router};

use crate::{mid, ArcAppState};

use super::{ping, subject};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/", ping(state.clone()))
        .nest("/subject", subject_init(state.clone()))
        .layer(middleware::from_extractor_with_state::<
            mid::UserAuth,
            ArcAppState,
        >(state.clone()))
}

fn ping(state: ArcAppState) -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .with_state(state)
}

fn subject_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/top", get(subject::top))
        .route("/", get(subject::list))
        .with_state(state)
}
