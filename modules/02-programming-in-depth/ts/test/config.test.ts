import { loadConfig } from "../src/config";
import { describe, it, expect, beforeEach, afterEach } from "vitest";

const ORIGINAL_ENV = process.env;

describe("config", () => {
  beforeEach(() => {
    process.env = { ...ORIGINAL_ENV };
  });
  afterEach(() => {
    process.env = ORIGINAL_ENV;
  });

  it("uses defaults when env not set", () => {
    delete process.env.PORT;
    delete process.env.APP_ENV;
    delete process.env.LOG_LEVEL;
    const cfg = loadConfig();
    expect(cfg.port).toBe(3000);
    expect(cfg.env).toBe("dev");
    expect(cfg.logLevel).toBe("info");
  });

  it("reads values from env", () => {
    process.env.PORT = "4000";
    process.env.APP_ENV = "test";
    process.env.LOG_LEVEL = "debug";
    const cfg = loadConfig();
    expect(cfg.port).toBe(4000);
    expect(cfg.env).toBe("test");
    expect(cfg.logLevel).toBe("debug");
  });
});
