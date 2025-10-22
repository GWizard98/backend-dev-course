import { buildApp } from "./server";

const app = buildApp();
const port = Number(process.env.PORT || 3000);
app
  .listen({ port, host: "127.0.0.1" })
  .then(() => app.log.info({ port }, "listening"))
  .catch((err) => {
    app.log.error(err, "failed to start");
    process.exit(1);
  });
