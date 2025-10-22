import Fastify from "fastify";
import { normalizeUserInput } from "./lib/normalizeUserInput.js";

export function buildApp() {
  const app = Fastify({ logger: true });

  app.post("/normalize", async (req, reply) => {
    const body = req.body ?? {};
    const input = String(body.input || "").trim();
    if (!input) {
      return reply.code(400).send({ error: "input must not be empty" });
    }
    const normalized = normalizeUserInput(input);
    return { normalized };
  });

  return app;
}
