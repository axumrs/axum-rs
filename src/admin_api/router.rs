use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::middleware::admin_auth;

pub fn init() -> Router {
    let subject_router = Router::new()
        .route(
            "/",
            get(super::subject::list)
                .post(super::subject::add)
                .put(super::subject::update),
        )
        .route(
            "/:id",
            get(super::subject::find)
                .delete(super::subject::del)
                .patch(super::subject::restore),
        );

    let topic_router = Router::new()
        .route(
            "/",
            get(super::topic::list)
                .post(super::topic::add)
                .put(super::topic::edit),
        )
        .route(
            "/:id",
            get(super::topic::find)
                .delete(super::topic::del)
                .patch(super::topic::restore),
        );

    let tag_router = Router::new()
        .route(
            "/",
            get(super::tag::list)
                .post(super::tag::add)
                .put(super::tag::edit),
        )
        .route(
            "/:id",
            get(super::tag::find)
                .delete(super::tag::del)
                .patch(super::tag::restore),
        );

    let admin_router = Router::new()
        .route(
            "/",
            get(super::admin::list)
                .post(super::admin::add)
                .put(super::admin::edit),
        )
        .route(
            "/:id",
            get(super::admin::find)
                .delete(super::admin::del)
                .patch(super::admin::restore),
        );

    let user_router = Router::new()
        .route(
            "/",
            get(super::user::list)
                .post(super::user::add)
                .put(super::user::edit),
        )
        .route(
            "/:id",
            get(super::user::find)
                .delete(super::user::del)
                .patch(super::user::restore),
        )
        .route("/freeze/:id", put(super::user::freeze))
        .route("/active/:id", put(super::user::active))
        .route("/pending/:id", put(super::user::pending))
        .route("/online/:email", get(super::user::online_drive))
        .route("/login_log/:id", get(super::user::login_log));

    let order_router = Router::new()
        .route("/", get(super::order::list))
        .route("/:id", get(super::order::find));

    let pay_apply_router = Router::new()
        .route("/:order_id/:user_id", get(super::pay_apply::find))
        .route("/reject", post(super::pay_apply::reject))
        .route("/accept", post(super::pay_apply::accept));

    let purchased_service_router = Router::new()
        .route("/", get(super::user_purchased_service::list))
        .route("/:id", get(super::user_purchased_service::find));

    Router::new()
        .nest("/subject", subject_router)
        .nest("/topic", topic_router)
        .nest("/tag", tag_router)
        .nest("/admin", admin_router)
        .nest("/user", user_router)
        .nest("/order", order_router)
        .nest("/pay_apply", pay_apply_router)
        .nest("/purchased_service", purchased_service_router)
        .layer(middleware::from_fn(admin_auth))
}
