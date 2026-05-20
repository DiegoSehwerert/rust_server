use axum::{routing::get, Router};

// Esta función empaqueta todas las rutas de este archivo
pub fn router() -> Router {
    Router::new()
        .route("/ping", get(ping_handler))
        .route("/version", get(version_handler))
}

// Endpoint: /api/v1/status/ping
async fn ping_handler() -> &'static str {
    "pong 🏓"
}

// Endpoint: /api/v1/status/version
async fn version_handler() -> &'static str {
    "v1.0.0 - Driver Dev Edition 🚀"
}