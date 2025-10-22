use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};

mod libmod {
    pub use crate::normalize_user_input;
}

#[derive(Clone, Default)]
struct AppState;

#[derive(Deserialize)]
struct NormalizeRequest {
    input: String,
}

#[derive(Serialize)]
struct NormalizeResponse {
    normalized: String,
}

#[tokio::main]
async fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let state = AppState::default();
    let app = Router::new()
        .route("/normalize", post(normalize))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!(%addr, "listening");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn normalize(State(_state): State<AppState>, Json(payload): Json<NormalizeRequest>) -> Result<Json<NormalizeResponse>, (axum::http::StatusCode, String)> {
    let input = payload.input.trim();
    if input.is_empty() {
        return Err((axum::http::StatusCode::BAD_REQUEST, "input must not be empty".into()));
    }
    let normalized = crate::normalize_user_input(input);
    Ok(Json(NormalizeResponse { normalized }))
}
