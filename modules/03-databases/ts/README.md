# Track: TypeScript (Prisma or Drizzle)

Recommended: PostgreSQL with Prisma (beginner-friendly) or Drizzle ORM.

Setup (Prisma)
- npm i -D prisma && npm i @prisma/client
- npx prisma init (creates prisma/schema.prisma and .env)
- Set DATABASE_URL in prisma/.env or project env
- npx prisma migrate dev --name init (or: npx prisma db push)

Setup (Drizzle)
- npm i drizzle-orm pg dotenv
- npm i -D drizzle-kit
- Configure drizzle.config.ts; run: npx drizzle-kit generate && npx drizzle-kit migrate

Local development
- Build: npx tsc -p .
- Lint: npx eslint .
- Test all: npx vitest run
- Run app/service: npm run dev (ensure DB env set)
