export function normalizeUserInput(s) {
  return String(s).trim().replace(/\s+/g, " ");
}
