use axum::{http::Request, routing::post, Json, Router};
use axum::body::Body;
use axum::extract::State;
use axum::middleware::{from_fn, Next};
use axum::response::Response;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct AppState;

#[derive(Deserialize)]
struct NormalizeRequest {
    input: String,
}

#[derive(Serialize)]
struct NormalizeResponse {
    normalized: String,
}

async fn normalize(
    State(_state): State<AppState>,
    Json(payload): Json<NormalizeRequest>,
) -> Result<Json<NormalizeResponse>, (axum::http::StatusCode, String)> {
    let input = payload.input.trim();
    if input.is_empty() {
        return Err((axum::http::StatusCode::BAD_REQUEST, "input must not be empty".into()));
    }
    let normalized = crate::normalize_user_input(input);
    Ok(Json(NormalizeResponse { normalized }))
}

async fn request_id(req: axum::http::Request<Body>, next: Next) -> Response {
    let id = Uuid::new_v4().to_string();
    let mut res = next.run(req).await;
    res.headers_mut().insert(
        axum::http::header::HeaderName::from_static("x-request-id"),
        axum::http::HeaderValue::from_str(&id).unwrap(),
    );
    res
}

pub fn router() -> Router {
    let state = AppState::default();
    Router::new()
        .route("/normalize", post(normalize))
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
                tracing::info_span!("req", method = ?req.method(), path = %req.uri())
            }),
        )
        .layer(from_fn(request_id))
        .with_state(state)
}
