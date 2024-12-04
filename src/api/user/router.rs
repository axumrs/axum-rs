use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::{mid, ArcAppState};

use super::{announcement, order, pay, ping, read_history, service, subject, tag, topic, user};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/", ping(state.clone()))
        .nest("/subject", subject_init(state.clone()))
        .nest("/topic", topic_init(state.clone()))
        .nest("/tag", tag_init(state.clone()))
        .nest("/user", user_init(state.clone()))
        .nest("/service", service_init(state.clone()))
        .nest("/order", order_init(state.clone()))
        .nest("/pay", pay_init(state.clone()))
        .nest("/read-history", read_history_init(state.clone()))
        .nest("/announcement", announcement_init(state.clone()))
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
        .route("/slug/:id", get(subject::get_slug))
        .route("/purchased", get(subject::purchased))
        .route("/is-purchased/:id", get(subject::is_purchased))
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
        .route("/password", put(user::change_pwd))
        .route("/profile", put(user::update_profile))
        .with_state(state)
}

fn service_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(service::list))
        .route("/subject/:subject_id", get(service::find_by_subject))
        .with_state(state)
}

fn order_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(order::list).post(order::create))
        .route("/:id", get(order::detail).put(order::cancel))
        .with_state(state)
}

fn pay_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", post(pay::add).put(pay::complete))
        .route("/order/:order_id", get(pay::detail_by_order))
        .with_state(state)
}

fn read_history_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(read_history::list))
        .with_state(state)
}

fn announcement_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/", get(announcement::list))
        .route("/:id", get(announcement::detail))
        .with_state(state)
}
