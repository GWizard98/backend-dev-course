# Module 03 Labs (Shared Specs)

These labs are language-agnostic; use your track’s DB client/ORM.
Target DB: PostgreSQL (recommended). You may use SQLite for convenience where noted.

Schema context: basic ecommerce (users, products, orders, order_items).
See labs/schema.sql for a starting point.

## Lab 01: SQL Fundamentals
- Objectives: Write SELECT/INSERT/UPDATE/DELETE; JOINs; filtering; aggregation.
- Tasks:
  - Seed users/products; write queries for top-N products, user order history, revenue per day.
- Acceptance Criteria:
  - Queries return correct rows on provided seed data; include tests or query outputs.

## Lab 02: Indexing and Query Plans
- Objectives: Add appropriate indexes; read EXPLAIN/EXPLAIN ANALYZE.
- Tasks:
  - Identify slow queries; propose and apply indexes; compare plans and timings.
- Acceptance Criteria:
  - Before/after plan snippets and timing improvement documented.

## Lab 03: Transactions and Constraints
- Objectives: Demonstrate ACID; enforce referential integrity; handle failures.
- Tasks:
  - Implement a multi-statement order creation within a transaction; simulate an error to verify rollback.
- Acceptance Criteria:
  - Tests/logs prove atomicity; constraints prevent invalid data (FKs, CHECKs, UNIQUE).

## Lab 04: Migrations and ORM
- Objectives: Introduce migrations and a simple repository pattern.
- Tasks:
  - Add a migration to include a status column and backfill; implement repository functions in your track.
- Acceptance Criteria:
  - Migration is reversible; repository functions covered by unit/integration tests.
