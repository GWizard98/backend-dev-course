use axum::{body::Body, http::Request, body};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn normalize_returns_400_on_empty() {
    let app = module02_rust_track::app::router();

    let req = Request::post("/normalize")
        .header("content-type", "application/json")
        .body(Body::from("{\"input\":\"   \"}"))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn normalize_returns_normalized_text() {
    let app = module02_rust_track::app::router();

    let req = Request::post("/normalize")
        .header("content-type", "application/json")
        .body(Body::from("{\"input\":\" Foo\\tBar\\nBaz \"}"))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), 200);

    let bytes = body::to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["normalized"], "Foo Bar Baz");
}
