import Fastify from "fastify";
import pino from "pino";
import { normalizeUserInput } from "./lib/normalizeUserInput";

const log = pino({ level: process.env.LOG_LEVEL || "info" });
const app = Fastify({ logger: log });

app.post("/normalize", async (req, reply) => {
  const body = (req.body ?? {}) as { input?: string };
  const input = (body.input || "").trim();
  if (!input) {
    return reply.code(400).send({ error: "input must not be empty" });
  }
  const normalized = normalizeUserInput(input);
  return { normalized };
});

const port = Number(process.env.PORT || 3000);
app
  .listen({ port, host: "127.0.0.1" })
  .then(() => log.info({ port }, "listening"))
  .catch((err) => {
    log.error(err, "failed to start");
    process.exit(1);
  });
