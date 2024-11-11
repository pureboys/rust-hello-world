use axum::Router;
use axum::routing::{get, post};
use crate::AppState;
use crate::service::tickets;

pub fn router(app_state: AppState) -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
        .nest("/api", Router::new()
            .route("/tickets", post(tickets::create_tickets).get(tickets::list_tickets))
            .route("/ticket/:id", get(tickets::get_ticket)),
        )
        .with_state(app_state)
}

