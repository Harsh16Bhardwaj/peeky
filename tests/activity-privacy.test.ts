import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const runtime = readFileSync("src-tauri/src/lib.rs", "utf8");
const build = readFileSync("scripts/build-release.ps1", "utf8");

describe("application-only activity boundary", () => {
  it("records every foreground program as an application source", () => {
    expect(runtime).toContain("kind: ActivitySourceKind::Application");
    expect(runtime).not.toContain("BrowserBridgeState");
  });

  it("does not package a browser extension or native host", () => {
    expect(build).not.toContain("PEEKY_EXTENSION_ID");
    expect(build).not.toContain("PeekyBrowserHost");
    expect(build).not.toContain("browser-extension");
  });
});
