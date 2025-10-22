import { buildApp } from "./server.js";

import { loadConfig } from "./config.js";

const cfg = loadConfig();
const app = buildApp();
app
  .listen({ port: cfg.port, host: "127.0.0.1" })
  .then(() => console.log(`listening on ${cfg.port} env=${cfg.env}`))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
