use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::{mid, ArcAppState};

use super::{
    announcement, order, profile, promotion, service, session, statistics, subject, tag, topic,
    user,
};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/subject", subject_init(state.clone()))
        .nest("/tag", tag_init(state.clone()))
        .nest("/topic", topic_init(state.clone()))
        .nest("/profile", profile_init(state.clone()))
        .nest("/session", session_init(state.clone()))
        .nest("/user", user_init(state.clone()))
        .nest("/service", service_init(state.clone()))
        .nest("/order", order_init(state.clone()))
        .nest("/statistics", statistics_init(state.clone()))
        .nest("/announcement", announcement_init(state.clone()))
        .nest("/promotion", promotion_init(state.clone()))
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
        .route("/{id}", delete(subject::del).patch(subject::res))
        .route("/all", get(subject::all))
        .with_state(state)
}

fn tag_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(tag::list).post(tag::add).put(tag::edit))
        .route("/all", get(tag::all))
        .route("/{id}", delete(tag::real_del))
        .with_state(state)
}

fn topic_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", post(topic::add).put(topic::edit).get(topic::list))
        .route("/{id}", delete(topic::del).patch(topic::res))
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
        .route("/{id}", delete(user::del).get(user::find_by_id))
        .route("/search", get(user::search))
        .with_state(state)
}

fn service_init(state: ArcAppState) -> Router {
    Router::new()
        .route(
            "/",
            get(service::list)
                .post(service::add)
                .put(service::edit)
                .patch(service::import),
        )
        .route(
            "/{id}",
            put(service::on_off)
                .delete(service::del)
                .patch(service::sync),
        )
        .route("/search", get(service::search))
        .route("/all", get(service::list_all))
        .with_state(state)
}

fn order_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(order::list).post(order::add).put(order::edit))
        .route("/pay/{order_id}", get(order::find_pay))
        .route("/{id}", put(order::close))
        .with_state(state)
}

fn statistics_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(statistics::index))
        .with_state(state)
}

fn announcement_init(state: ArcAppState) -> Router {
    Router::new()
        .route(
            "/",
            get(announcement::list)
                .post(announcement::add)
                .put(announcement::edit),
        )
        .route("/{id}", delete(announcement::del))
        .with_state(state)
}

fn promotion_init(state: ArcAppState) -> Router {
    Router::new()
        .route(
            "/",
            get(promotion::list)
                .post(promotion::create)
                .put(promotion::edit),
        )
        .route("/{id}", delete(promotion::del))
        .with_state(state)
}
