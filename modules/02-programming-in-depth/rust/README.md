# Track: Rust

Suggested stack: Axum + Tokio. Error handling: anyhow/thiserror. Logging: tracing.

## Local development
- Build: cargo build
- Lint: cargo clippy -- -D warnings
- Test all: cargo test
- Test single file/case: cargo test <name>
- Run app/service: cargo run

## Labs
Follow the shared specs in ../LABS.md. Provide an Axum implementation with unit tests and tracing.
