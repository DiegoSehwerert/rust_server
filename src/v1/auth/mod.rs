use crate::v1::auth::handlers::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;

pub mod handlers; // 👈 ¡OJO! Ponle 'pub' aquí delante para que deje de ser privado

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", get(handlers::login))
        .route("/callback", get(handlers::callback))
        .with_state(state)
}
