use axum::{routing::post, Router};

pub fn init() -> Router {
    let admin_router = Router::new().route("/login", post(super::admin::login));

    let user_router = Router::new()
        .route("/register", post(super::user::register))
        .route("/login", post(super::user::login));

    Router::new()
        .nest("/admin", admin_router)
        .nest("/user", user_router)
}
