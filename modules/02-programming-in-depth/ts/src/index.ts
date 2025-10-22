import { buildApp } from "./server";
import { loadConfig } from "./config";

const cfg = loadConfig();
const app = buildApp();
app
  .listen({ port: cfg.port, host: "127.0.0.1" })
  .then(() => app.log.info({ port: cfg.port, env: cfg.env }, "listening"))
  .catch((err) => {
    app.log.error(err, "failed to start");
    process.exit(1);
  });
