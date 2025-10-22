import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { loadConfig } from "../config.js";

const ORIGINAL_ENV = process.env;

describe("config (js)", () => {
  beforeEach(() => {
    process.env = { ...ORIGINAL_ENV };
  });
  afterEach(() => {
    process.env = ORIGINAL_ENV;
  });

  it("defaults", () => {
    delete process.env.PORT;
    delete process.env.APP_ENV;
    delete process.env.LOG_LEVEL;
    const cfg = loadConfig();
    expect(cfg.port).toBe(3000);
    expect(cfg.env).toBe("dev");
    expect(cfg.logLevel).toBe("info");
  });

  it("reads env", () => {
    process.env.PORT = "5001";
    process.env.APP_ENV = "test";
    process.env.LOG_LEVEL = "debug";
    const cfg = loadConfig();
    expect(cfg.port).toBe(5001);
    expect(cfg.env).toBe("test");
    expect(cfg.logLevel).toBe("debug");
  });
});
