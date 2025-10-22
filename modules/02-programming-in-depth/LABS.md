# Module 02 Labs (Shared Specs)

These labs are language-agnostic. Implement in your chosen track under this module.

## Lab 01: Testing Basics
- Objectives: Write unit tests; set up a test runner; practice TDD on a small function.
- Tasks:
  - Implement a pure function (e.g., `normalize_user_input`) with edge cases.
  - Write tests covering happy path, edge cases, and failure cases.
- Acceptance Criteria:
  - Tests run via the track's test command and all pass.
  - Clear naming and assertions; negative cases included.

## Lab 02: Error Handling
- Objectives: Propagate and categorize errors; return appropriate HTTP status codes.
- Tasks:
  - Add an endpoint that validates input and returns 400 on validation errors.
  - Introduce a service layer that can return typed/domain errors.
- Acceptance Criteria:
  - Distinct error types mapped to correct responses; unit tests assert mapping.

## Lab 03: Logging and Tracing
- Objectives: Add structured logs and request-scoped correlation; basic tracing spans.
- Tasks:
  - Log at INFO for normal ops and WARN/ERROR for failures; include request ID.
  - Add a simple middleware/interceptor to time requests.
- Acceptance Criteria:
  - Logs are structured (JSON or key=value); request ID present; timings captured.

## Lab 04: Configuration and Environments
- Objectives: Load config from env and files; support dev/test separation.
- Tasks:
  - Implement config loader with defaults + env overrides (env vars: PORT, APP_ENV, LOG_LEVEL).
  - Ensure tests can inject config without relying on global state.
- Acceptance Criteria:
  - Running with different env values changes behavior as expected; tests cover config.
