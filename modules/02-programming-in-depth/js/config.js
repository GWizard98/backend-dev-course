export function loadConfig() {
  const port = Number(process.env.PORT || 3000);
  const env = String(process.env.APP_ENV || "dev");
  const logLevel = String(process.env.LOG_LEVEL || "info");
  return { port, env, logLevel };
}
