# Track: Go (sqlx or GORM)

Recommended: PostgreSQL with jmoiron/sqlx (lightweight) or GORM (feature-rich).

Setup (sqlx)
- go get github.com/jmoiron/sqlx github.com/lib/pq
- Env: export DATABASE_URL=postgres://user:pass@localhost:5432/bdcourse?sslmode=disable
- Open connection with sqlx.Open("postgres", os.Getenv("DATABASE_URL"))

Setup (GORM)
- go get gorm.io/gorm gorm.io/driver/postgres
- DSN via env; auto-migrate models as needed

Migrations
- Install: brew install golang-migrate
- Create/run: migrate create -ext sql -dir db/migrations -seq init && migrate -path db/migrations -database $DATABASE_URL up

Local development
- Lint: go vet ./...
- Test all: go test ./...
- Run app/service: go run ./cmd/app
