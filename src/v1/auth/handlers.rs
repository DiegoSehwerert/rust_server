use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// 1. Centralizamos la configuración en una estructura limpia
pub struct AppState {
    pub github_client_id: String,
    pub github_client_secret: String,
    pub http_client: reqwest::Client,
}

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
}

#[derive(Deserialize)]
struct GitHubTokenResponse {
    access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub email: Option<String>,
}

#[derive(Serialize)]
struct TokenPayload<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
}

// PASO 1: Redirigir al usuario
pub async fn login(State(state): State<Arc<AppState>>) -> Redirect {
    let github_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&scope=user:email",
        state.github_client_id
    );
    Redirect::to(&github_url)
}

// PASO 2: Callback
pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CallbackParams>,
) -> Result<impl IntoResponse, StatusCode> {
    // 1. Intercambiar código por Token de manera segura usando el operador '?'
    let token_request = state
        .http_client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&TokenPayload {
            client_id: &state.github_client_id,
            client_secret: &state.github_client_secret,
            code: &params.code,
        })
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Si falla la red, devuelve HTTP 500

    let token_data = token_request
        .json::<GitHubTokenResponse>()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?; // Si GitHub devuelve basura, HTTP 502

    // 2. Pedir los datos del perfil del usuario
    let user_request = state
        .http_client
        .get("https://api.github.com/user")
        .header(
            "Authorization",
            format!("token {}", token_data.access_token),
        )
        .header("User-Agent", "mi-servidor-axum-loquete")
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let github_user = user_request
        .json::<GitHubUser>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?; // Si el token no vale, HTTP 401

    println!("🔥 ¡Usuario autenticado sin riesgos!: {:?}", github_user);

    Ok(Json(github_user))
}
