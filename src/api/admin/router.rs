use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::{mid, ArcAppState};

use super::{profile, session, subject, tag, topic, user};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/subject", subject_init(state.clone()))
        .nest("/tag", tag_init(state.clone()))
        .nest("/topic", topic_init(state.clone()))
        .nest("/profile", profile_init(state.clone()))
        .nest("/session", session_init(state.clone()))
        .nest("/user", user_init(state.clone()))
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
    Router::new()
        .route("/", get(tag::list).post(tag::add).put(tag::edit))
        .route("/all", get(tag::all))
        .route("/:id", delete(tag::real_del))
        .with_state(state)
}

fn topic_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", post(topic::add).put(topic::edit).get(topic::list))
        .route("/:id", delete(topic::del).patch(topic::res))
        .with_state(state)
}

fn profile_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/change-pwd", put(profile::change_pwd))
        .with_state(state)
}

fn session_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/logout", delete(session::logout))
        .with_state(state)
}

fn user_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(user::list).post(user::add).put(user::edit))
        .with_state(state)
}
