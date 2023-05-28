use axum::{
    middleware::from_extractor,
    routing::{get, post},
    Router,
};

use crate::middleware::{PurchaseSubject, UserAuth, UserInfoOption, UserReadHistory};

pub fn init() -> Router {
    let subject_detail_router = Router::new()
        .route("/:slug", get(super::subject::detail))
        .layer(from_extractor::<PurchaseSubject>());

    let subject_router = Router::new()
        .route("/top4", get(super::subject::top4))
        .route("/", get(super::subject::list))
        .nest("/", subject_detail_router);

    let topic_detail_router = Router::new()
        .route("/:subject_slug/:slug", get(super::topic::detail))
        .layer(from_extractor::<PurchaseSubject>())
        .layer(from_extractor::<UserReadHistory>());
    let topic_router = Router::new()
        .route("/top10", get(super::topic::top10))
        .route("/", get(super::topic::list))
        .nest("/", topic_detail_router);

    let tag_router = Router::new()
        .route("/", get(super::tag::list))
        .route("/:name", get(super::tag::detail));

    let user_router = Router::new()
        .route("/online_derive", get(super::user::online_derive))
        .route("/login_log", get(super::user::login_log))
        .route("/logout", get(super::user::logout))
        .route("/order", get(super::order::list).post(super::order::create))
        .route("/order/:id", get(super::order::find))
        .route("/order/pay", post(super::order::pay))
        .route("/info", get(super::user::basic_info))
        .route("/check-in", get(super::user::check_in))
        .route("/pay-apply", post(super::pay_apply::add))
        .route("/pay-apply/:order_id", get(super::pay_apply::find))
        .route("/pay/:id", get(super::pay::find))
        .route(
            "/profile",
            get(super::user::profile).post(super::user::update_profile),
        )
        .route("/change-pwd", post(super::user::change_pwd))
        .route("/history", get(super::user_read_history::list))
        .layer(from_extractor::<UserAuth>());

    Router::new()
        .nest("/subject", subject_router)
        .nest("/topic", topic_router)
        .nest("/tag", tag_router)
        .nest("/user", user_router)
        .layer(from_extractor::<UserInfoOption>())
}
