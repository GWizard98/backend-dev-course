# Track: Rust (SQLx)

Recommended: PostgreSQL with SQLx (async) and sqlx-cli for migrations.

Setup
- Dependencies: cargo add sqlx --features "runtime-tokio,postgres,macros" tokio --features "macros,rt-multi-thread"
- CLI (migrations): cargo install sqlx-cli --no-default-features --features native-tls,postgres
- Env: export DATABASE_URL=postgres://user:pass@localhost:5432/bdcourse

Workflow
- Create DB: sqlx database create
- Apply migrations: sqlx migrate run
- Generate query metadata (offline): cargo sqlx prepare -- --lib

Local development
- Build: cargo build
- Lint: cargo clippy -- -D warnings
- Test all: cargo test
- Run app/service: cargo run
