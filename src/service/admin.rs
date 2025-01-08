use axum::extract::Request;
use axum::http;
use axum::middleware::Next;
use axum::response::Response;

pub async fn admin_handler() -> &'static str {
    "Admin Page"
}


pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, Response> {
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
