use axum::{middleware, routing::get, Router};

use crate::{mid, ArcAppState};

use super::subject;

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/subject", subject_init(state.clone()))
        .layer(middleware::from_extractor_with_state::<
            mid::AdminAuth,
            ArcAppState,
        >(state.clone()))
}

fn subject_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(subject::list))
        .with_state(state)
}
