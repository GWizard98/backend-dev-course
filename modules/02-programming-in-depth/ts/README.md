# Track: TypeScript (Node.js)

Suggested stack: Fastify or Express. Testing: Vitest/Jest. Logging: pino. Runtime: ts-node or build to dist/.

## Local development
- Build: npx tsc -p .
- Lint: npx eslint .
- Test all: npx vitest run (or: npx jest)
- Test single file/case: npx vitest run -t "<name>" (or: npx jest -t "<name>")
- Run app/service: node dist/index.js (or: npx ts-node src/index.ts)

## Labs
Follow the shared specs in ../LABS.md. Implement with Fastify/Express and pino logging.
