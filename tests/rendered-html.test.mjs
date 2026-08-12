import assert from "node:assert/strict";
import test from "node:test";

const templateRoot = new URL("../", import.meta.url);

async function render(path = "/") {
  const workerUrl = new URL("../dist/server/index.js", import.meta.url);
  workerUrl.searchParams.set("test", `${process.pid}-${Date.now()}`);
  const { default: worker } = await import(workerUrl.href);

  return worker.fetch(
    new Request(`http://localhost${path}`, { headers: { accept: "text/html" } }),
    { ASSETS: { fetch: async () => new Response("Not found", { status: 404 }) } },
    { waitUntil() {}, passThroughOnException() {} },
  );
}

test("server-renders the Peeky product home page", async () => {
  const response = await render();
  assert.equal(response.status, 200);
  assert.match(response.headers.get("content-type") ?? "", /^text\/html\b/i);
  const html = await response.text();
  assert.match(html, /Your screen is intense/);
  assert.match(html, /Download for Windows/);
  assert.match(html, /Private means private/);
  assert.doesNotMatch(html, /codex-preview|loading skeleton|react-loading-skeleton/i);
});

test("renders dedicated download and privacy pages", async () => {
  const [downloadResponse, privacyResponse] = await Promise.all([render("/download"), render("/privacy")]);
  assert.equal(downloadResponse.status, 200);
  assert.equal(privacyResponse.status, 200);
  assert.match(await downloadResponse.text(), /Pick your Peeky/);
  assert.match(await privacyResponse.text(), /Local by design/);
});

test("keeps release artifacts in the deployable public tree", async () => {
  const { access } = await import("node:fs/promises");
  await Promise.all([
    access(new URL("public/downloads/Peeky-Setup-x64.exe", templateRoot)),
    access(new URL("public/downloads/Peeky-Portable-x64.zip", templateRoot)),
    access(new URL("public/downloads/SHA256SUMS.txt", templateRoot)),
    access(new URL("public/og.png", templateRoot)),
  ]);
});
