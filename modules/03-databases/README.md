# Module 03: Databases

## Objectives
- Understand relational vs. NoSQL trade-offs and when to use each
- Model data, design schemas, and write idiomatic SQL (DDL/DML)
- Apply indexing, transactions, and migrations
- Use a language-appropriate DB client/ORM to implement queries safely

## Prerequisites
- Module 01–02 concepts (CLI, git, HTTP, testing)

## Lecture Notes (outline)
- Data modeling: entities, relationships, normalization vs. pragmatism
- SQL fundamentals: SELECT/INSERT/UPDATE/DELETE, JOINs, GROUP BY
- Indexes: types, query plans, performance considerations
- Transactions and isolation; ACID; optimistic vs. pessimistic locking
- Migrations and schema versioning; rollback strategy

## Tracks (suggested libraries)
- Rust: SQLx or Diesel
- TypeScript/JavaScript: Prisma, Drizzle, or Knex
- Python: SQLAlchemy
- Go: database/sql + sqlx or GORM

## Labs
- See LABS.md for shared lab specs and acceptance criteria

## Assessments
- Quiz: SQL and indexing fundamentals
- Practical: implement queries and migrations for a small schema
