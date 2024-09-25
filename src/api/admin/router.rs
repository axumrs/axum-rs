use axum::{middleware, routing::get, Router};

use crate::{mid, ArcAppState};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/", ping(state.clone()))
        .layer(middleware::from_extractor_with_state::<
            mid::AdminAuth,
            ArcAppState,
        >(state.clone()))
}

fn ping(state: ArcAppState) -> Router {
    Router::new()
        .route("/ping", get(|| async { "PONG" }))
        .with_state(state)
}
