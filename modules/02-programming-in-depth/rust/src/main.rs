use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};

use module02_rust_track::app;

#[tokio::main]
async fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let app = app::router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
