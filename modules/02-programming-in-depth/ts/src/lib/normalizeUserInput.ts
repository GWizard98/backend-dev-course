export function normalizeUserInput(s: string): string {
  return s.trim().replace(/\s+/g, " ");
}
