use axum::Router;
use std::env;
use std::sync::Arc;
use v1::auth::handlers::AppState; // Importamos tu estructura de estado de siempre

mod v1;

#[tokio::main]
async fn main() {
    // 1. Cargamos el archivo .env
    dotenvy::dotenv().expect("❌ No se pudo cargar el archivo .env");

    // 2. Creamos el AppState SÓLO con lo necesario para GitHub y HTTP
    let state = Arc::new(AppState {
        github_client_id: env::var("GITHUB_CLIENT_ID").expect("Falta GITHUB_CLIENT_ID"),
        github_client_secret: env::var("GITHUB_CLIENT_SECRET").expect("Falta GITHUB_CLIENT_SECRET"),
        http_client: reqwest::Client::new(),
    });

    // 3. Leemos el puerto
    let puerto = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let direccion = format!("127.0.0.1:{}", puerto);

    // 4. Pasamos el estado de GitHub simplificado a las rutas
    let app = Router::new().nest("/api/v1", v1::routes(Arc::clone(&state)));

    // 5. Encendemos el servidor
    let listener = tokio::net::TcpListener::bind(&direccion).await.unwrap();
    println!("🚀 Servidor simplificado corriendo en http://{}", direccion);

    axum::serve(listener, app).await.unwrap();
}
