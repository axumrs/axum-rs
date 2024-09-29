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
        .nest("/admin", admin::router::init(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(state.cfg.upload.max_size))
        .layer(from_extractor::<mid::IpAndUserAgent>());

    Router::new().nest(&state.cfg.web.prefix, r)
}

fn web_init(state: ArcAppState) -> Router {
    Router::new().route("/", get(web::ping)).with_state(state)
}

fn auth_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/register-send-code", post(auth::register_send_code))
        .route("/admin-login", post(auth::admin_login))
        .with_state(state)
}
