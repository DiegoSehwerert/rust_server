use crate::v1::auth::handlers::AppState;
use axum::{Router, routing::get};
use std::sync::Arc; // Importamos el estado global

pub mod handlers; // Lo hacemos público para evitar dolores de cabeza de visibilidad

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/profile", get(handlers::profile))
        .with_state(state) // 🔥 Le inyectamos el estado al módulo de usuarios
}
