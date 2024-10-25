use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::{mid, ArcAppState};

use super::{ping, service, subject, tag, topic, user};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/", ping(state.clone()))
        .nest("/subject", subject_init(state.clone()))
        .nest("/topic", topic_init(state.clone()))
        .nest("/tag", tag_init(state.clone()))
        .nest("/user", user_init(state.clone()))
        .nest("/service", service_init(state.clone()))
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
        .route("/detail/:slug", get(subject::detail))
        .with_state(state)
}

fn topic_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/top", get(topic::top))
        .route("/", get(topic::list))
        .route("/detail/:subject_slug/:slug", get(topic::detail))
        .route("/protected-content", post(topic::get_protected_content))
        .with_state(state)
}

fn tag_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(tag::list))
        .route("/:name", get(tag::detail))
        .with_state(state)
}

fn user_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/logout", delete(user::logout))
        .route("/check-in", get(user::check_in))
        .route("/sessions", get(user::session_list))
        .route("/login-logs", get(user::login_log_list))
        .with_state(state)
}

fn service_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(service::list))
        .with_state(state)
}
