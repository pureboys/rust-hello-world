use axum::Router;
use axum::routing::{get, put};
use crate::AppState;
use crate::service::tickets;

pub fn router(app_state: AppState) -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
        .nest("/api", Router::new()
            .route("/tickets", put(tickets::create_tickets).get(tickets::list_tickets).post(tickets::update_ticket))
            .route("/ticket/:id", get(tickets::get_ticket)),
        )
        .with_state(app_state)
}

