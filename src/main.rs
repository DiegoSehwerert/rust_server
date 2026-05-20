use axum::Router;
use std::net::SocketAddr;

// Importamos la carpeta v1
mod v1;

#[tokio::main]
async fn main() {
    // 1. Construimos el Rúter Principal e inyectamos las rutas de la V1
    // Todas las rutas de v1 colgarán automáticamente de /api/v1
    let app = Router::new()
        .nest("/api/v1", v1::routes());

    // 2. Definimos la dirección IP y el puerto (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("🔥 Servidor arrancado en http://{}", addr);
    println!("Pistas de endpoints disponibles:");
    println!("  -> http://{}/api/v1/status/ping", addr);
    println!("  -> http://{}/api/v1/status/version", addr);

    // 3. Encendemos el servidor
    axum::serve(listener, app).await.unwrap();
}