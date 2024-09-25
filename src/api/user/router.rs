use axum::{middleware, routing::get, Router};

use crate::{mid, ArcAppState};

use super::ping;

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/", ping(state.clone()))
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
