import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import test from "node:test";

const root = new URL("../", import.meta.url);

test("builds all three static pages", async () => {
  const [home, download, privacy] = await Promise.all([
    readFile(new URL("dist/index.html", root), "utf8"),
    readFile(new URL("dist/download/index.html", root), "utf8"),
    readFile(new URL("dist/privacy/index.html", root), "utf8"),
  ]);

  assert.match(home, /Peeky - A calmer way to use your screen/);
  assert.match(download, /Download Peeky for Windows/);
  assert.match(privacy, /Privacy Policy - Peeky/);
});

test("copies release artifacts into the self-hostable build", async () => {
  await Promise.all([
    access(new URL("dist/downloads/Peeky-Setup-x64.exe", root)),
    access(new URL("dist/downloads/Peeky-Portable-x64.zip", root)),
    access(new URL("dist/downloads/SHA256SUMS.txt", root)),
    access(new URL("dist/og.png", root)),
  ]);
});

test("ships the complete product story in the client bundle", async () => {
  const { readdir } = await import("node:fs/promises");
  const assets = await readdir(new URL("dist/assets/", root));
  const script = assets.find((asset) => asset.endsWith(".js"));
  assert.ok(script, "expected a built JavaScript bundle");
  const bundle = await readFile(new URL(`dist/assets/${script}`, root), "utf8");
  assert.match(bundle, /Calm screen breaks/);
  assert.match(bundle, /A break should/);
  assert.match(bundle, /Does Peeky work offline/);
  assert.match(bundle, /What Peeky can store/);
});
