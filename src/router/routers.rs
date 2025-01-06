use crate::service::tickets;
use crate::AppState;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::{get, put};
use axum::{http, middleware, Router};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/admin", get(admin_handler))
        .route_layer(middleware::from_fn(auth_middleware))
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
        .with_state(app_state)
}

async fn admin_handler() -> &'static str {
    "Admin Page"
}


async fn auth_middleware(req: Request, next: Next) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header == "secret" {
            return Ok(next.run(req).await);
        }
    }

    Err(Response::builder()
        .status(http::StatusCode::UNAUTHORIZED)
        .body("Unauthorized".into())
        .unwrap())
}
