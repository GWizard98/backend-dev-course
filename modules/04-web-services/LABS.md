# Module 04 Labs (Shared Specs)

Build a small REST API for a simple resource: articles with CRUD and validation.
Add basic auth (optional) and OpenAPI docs where feasible.

Base resource: Article { id, title, body, author, created_at, updated_at }

## Lab 01: Routing and Handlers
- Endpoints: POST /articles, GET /articles, GET /articles/{id}
- Acceptance:
  - Proper status codes (201 on create, 200 on reads), Location header on create

## Lab 02: Validation and Error Mapping
- Title required (min length 3), body required (min length 10)
- Acceptance:
  - 400 with validation errors; consistent error shape; tests cover invalid inputs

## Lab 03: Update/Delete and Idempotency
- Endpoints: PUT /articles/{id}, DELETE /articles/{id}
- Acceptance:
  - 404 on missing id; updates return latest representation; delete idempotent (204 or 404 documented)

## Lab 04: OpenAPI Docs (optional for some tracks)
- Provide OpenAPI schema (code-first or YAML)
- Acceptance:
  - Endpoints/types documented and validated against examples
