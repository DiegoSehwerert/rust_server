use axum::Router;

// 1. Declaramos el archivo status.rs como sub-módulo
mod status;

// 2. Agrupamos todos los rúters de la versión 1
pub fn routes() -> Router {
    Router::new().nest("/status", status::router())
    // Cuando crees 'users', solo tendrás que añadir otra línea aquí:
    // .nest("/users", users::router())
}
