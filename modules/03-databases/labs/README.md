# Labs: setup and usage

Recommended: PostgreSQL 15+.

- Apply schema: psql $DATABASE_URL -f labs/schema.sql
- Seed minimal data (example):
  - INSERT INTO users(email,name) VALUES ('a@example.com','Alice'), ('b@example.com','Bob');
  - INSERT INTO products(sku,title,price_cents) VALUES ('SKU1','Widget',1299), ('SKU2','Gadget',2599);

For SQLite (alternative):
- Use sqlite3 db.sqlite < labs/schema.sql (adjust types if needed)

Deliverables:
- SQL files or code snippets per lab
- Brief README per track describing how to run queries/tests
