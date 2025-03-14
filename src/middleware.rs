use axum::{http::{Request, header}, middleware::Next, response::Response};
use crate::auth::verify_token;

pub async fn auth_middleware(
    req: Request<axum::body::Body>, 
    next: Next,                    
) -> Result<Response, axum::http::StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());
    
    if let Some(header) = auth_header {
        if let Some(token) = header.strip_prefix("Bearer ") {
            if let Some(user_id) = verify_token(token) {
                let mut req = req;
                req.extensions_mut().insert(user_id);
                return Ok(next.run(req).await);
            }
        }
    }
    Err(axum::http::StatusCode::UNAUTHORIZED)
}