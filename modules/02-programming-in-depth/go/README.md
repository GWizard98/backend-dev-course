# Track: Go

Suggested stack: net/http or chi. Testing: go test. Logging: slog or zerolog.

## Local development
- Build: go build ./...
- Lint: go vet ./... (or: golangci-lint run)
- Test all: go test ./...
- Test single file/case: go test ./... -run "TestName"
- Run app/service: go run ./cmd/app

## Labs
Follow the shared specs in ../LABS.md. Implement with net/http or chi; include table-driven tests.
