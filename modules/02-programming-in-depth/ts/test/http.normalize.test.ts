import { describe, it, expect } from "vitest";
import { buildApp } from "../src/server";

describe("POST /normalize", () => {
  it("400 on empty input", async () => {
    const app = buildApp();
    const res = await app.inject({ method: "POST", url: "/normalize", payload: { input: "   " } });
    expect(res.statusCode).toBe(400);
    expect(res.headers["x-request-id"]).toBeDefined();
  });
  it("200 with normalized text", async () => {
    const app = buildApp();
    const res = await app.inject({ method: "POST", url: "/normalize", payload: { input: " Foo\tBar\nBaz " } });
    expect(res.statusCode).toBe(200);
    expect(res.headers["x-request-id"]).toBeDefined();
    expect(res.json()).toEqual({ normalized: "Foo Bar Baz" });
  });
});
