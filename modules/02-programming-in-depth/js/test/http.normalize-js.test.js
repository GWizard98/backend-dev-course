import { describe, it, expect } from "vitest";
import { buildApp } from "../server.js";

describe("POST /normalize (JS)", () => {
  it("400 on empty input", async () => {
    const app = buildApp();
    const res = await app.inject({ method: "POST", url: "/normalize", payload: { input: "   " } });
    expect(res.statusCode).toBe(400);
  });
  it("200 with normalized text", async () => {
    const app = buildApp();
    const res = await app.inject({ method: "POST", url: "/normalize", payload: { input: " Foo\tBar\nBaz " } });
    expect(res.statusCode).toBe(200);
    expect(res.json()).toEqual({ normalized: "Foo Bar Baz" });
  });
});
