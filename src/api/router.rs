use axum::{
    extract::DefaultBodyLimit,
    middleware::from_extractor,
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
};

use crate::{mid, ArcAppState};

use super::{admin, auth, user, web};

pub fn init(state: ArcAppState) -> Router {
    let r = Router::new()
        .nest("/", web_init(state.clone()))
        .nest("/auth", auth_init(state.clone()))
        .nest("/user", user::router::init(state.clone()))
        .nest("/admin", admin::router::init(state.clone()));

    Router::new()
        .nest(&state.cfg.web.prefix, r)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(state.cfg.upload.max_size))
        .layer(from_extractor::<mid::IpAndUserAgent>())
}

fn web_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/ping", get(web::ping))
        .route("/rss", get(web::rss))
        .with_state(state)
}

fn auth_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/send-code", post(auth::send_code))
        .route("/admin-login", post(auth::admin_login))
        .route("/active", post(auth::active))
        .route("/reset-pwd", post(auth::reset_password))
        .with_state(state)
}
