use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};

mod app;

#[tokio::main]
async fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let app = app::router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!(%addr, "listening");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
