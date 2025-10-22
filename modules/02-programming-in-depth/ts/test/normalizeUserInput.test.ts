import { describe, it, expect } from "vitest";
import { normalizeUserInput } from "../src/lib/normalizeUserInput";

describe("normalizeUserInput", () => {
  it("trims and collapses spaces", () => {
    expect(normalizeUserInput("  Hello   world\n")).toBe("Hello world");
  });
  it("handles empty and spaces-only", () => {
    expect(normalizeUserInput("")).toBe("");
    expect(normalizeUserInput("   \t\n")).toBe("");
  });
  it("handles tabs and newlines", () => {
    expect(normalizeUserInput("Foo\tBar\nBaz")).toBe("Foo Bar Baz");
  });
});
