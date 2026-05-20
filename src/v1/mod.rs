use crate::v1::auth::handlers::AppState;
use axum::Router;
use std::sync::Arc;

// 1. 🔥 ¡OJO AQUÍ! Añadimos 'pub' delante de ambos módulos
// Esto le dice a Rust: "Cualquiera que entre a V1 puede ver que existen estas subcarpetas"
pub mod auth;
pub mod users;

/// Reúne todas las rutas de la versión 1 de la API.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        // Conectamos las rutas de autenticación pasándole el estado clonado
        .nest("/auth", auth::router(Arc::clone(&state)))
        // Conectamos las rutas de usuarios
        .nest("/users", users::router(Arc::clone(&state)))
}
