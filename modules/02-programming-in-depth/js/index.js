import Fastify from "fastify";
import { normalizeUserInput } from "./lib/normalizeUserInput.js";

const app = Fastify();

app.post("/normalize", async (req, reply) => {
  const body = (req.body ?? {}) as { input?: string };
  const input = String(body.input || "").trim();
  if (!input) {
    return reply.code(400).send({ error: "input must not be empty" });
  }
  const normalized = normalizeUserInput(input);
  return { normalized };
});

const port = Number(process.env.PORT || 3000);
app
  .listen({ port, host: "127.0.0.1" })
  .then(() => console.log(`listening on ${port}`))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
