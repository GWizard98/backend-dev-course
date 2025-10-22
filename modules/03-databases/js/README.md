# Track: JavaScript (Knex or Drizzle)

Recommended: PostgreSQL with Knex (simple) or Drizzle ORM.

Setup (Knex)
- npm i knex pg && npm i -D knex
- npx knex init (creates knexfile.js)
- Set DATABASE_URL in env or configure client/connection in knexfile.js
- Migrations: npx knex migrate:make init && npx knex migrate:latest

Setup (Drizzle)
- npm i drizzle-orm pg dotenv && npm i -D drizzle-kit
- Configure drizzle.config.ts/js; run: npx drizzle-kit generate && npx drizzle-kit migrate

Local development
- Lint: npx eslint .
- Test all: npx vitest run
- Run app/service: node index.js (ensure DB env set)
