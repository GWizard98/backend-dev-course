use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};

use module02_rust_track::{app, config};

#[tokio::main]
async fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let app = app::router();

    let cfg = config::Config::from_env();
    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    tracing::info!(%addr, env = %cfg.env, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
