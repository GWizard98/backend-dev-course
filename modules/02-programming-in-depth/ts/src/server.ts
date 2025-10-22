import Fastify from "fastify";
import pino from "pino";
import { normalizeUserInput } from "./lib/normalizeUserInput";

export function buildApp() {
  const log = pino({ level: process.env.LOG_LEVEL || "info" });
  const app = Fastify({ logger: log, genReqId: () => crypto.randomUUID() });

  app.post("/normalize", async (req, reply) => {
    const body = (req.body ?? {}) as { input?: string };
    const input = (body.input || "").trim();
    if (!input) {
      return reply.code(400).send({ error: "input must not be empty" });
    }
    const normalized = normalizeUserInput(input);
    return { normalized };
  });

  app.addHook("onRequest", async (req) => {
    req.log = req.log.child({ reqId: req.id });
  });

  app.addHook("onSend", async (req, reply, payload) => {
    reply.header("x-request-id", req.id);
    return payload;
  });

  return app;
}
