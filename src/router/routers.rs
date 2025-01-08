use crate::service::{admin, concur, tickets};
use crate::AppState;
use axum::routing::{get, put};
use axum::{middleware, Router};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/admin", get(admin::admin_handler))
        .route_layer(middleware::from_fn(admin::auth_middleware))
        .route("/", get(|| async { "Hello, World!" }))
        .nest(
            "/api",
            Router::new()
                .route(
                    "/tickets",
                    put(tickets::create_tickets)
                        .get(tickets::list_tickets)
                        .post(tickets::update_ticket),
                )
                .route("/ticket/:id", get(tickets::get_ticket)),
        )
        .nest(
            "/concur",
            Router::new().route("/:id", get(concur::get_concur)).route("/", get(concur::reqwest_url)),
        )
        .with_state(app_state)
}
