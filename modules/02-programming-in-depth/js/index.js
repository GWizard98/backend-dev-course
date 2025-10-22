import { buildApp } from "./server.js";

const app = buildApp();
const port = Number(process.env.PORT || 3000);
app
  .listen({ port, host: "127.0.0.1" })
  .then(() => console.log(`listening on ${port}`))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
