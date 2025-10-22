use axum::{body::Body, http::Request};
use module02_rust_track::normalize_user_input; // ensure the crate builds as a lib
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

    let bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["normalized"], "Foo Bar Baz");
}
