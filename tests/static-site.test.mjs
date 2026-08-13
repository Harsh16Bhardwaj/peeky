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
  assert.match(home, /canonical/);
  assert.match(home, /Your screen is intense/);
  assert.match(download, /Download Peeky for Windows/);
  assert.match(download, /Take a better/);
  assert.match(privacy, /Privacy Policy - Peeky/);
  assert.match(privacy, /What Peeky stores/);
});

test("copies release artifacts into the self-hostable build", async () => {
  await Promise.all([
    access(new URL("dist/downloads/Peeky-Setup-x64.exe", root)),
    access(new URL("dist/downloads/Peeky-Portable-x64.zip", root)),
    access(new URL("dist/downloads/SHA256SUMS.txt", root)),
    access(new URL("dist/og.png", root)),
    access(new URL("dist/og.jpg", root)),
  ]);
});

test("ships the complete product story in the client bundle", async () => {
  const { readdir } = await import("node:fs/promises");
  const assets = await readdir(new URL("dist/assets/", root));
  const scripts = assets.filter((asset) => asset.endsWith(".js"));
  assert.ok(scripts.length, "expected built JavaScript bundles");
  const bundle = (await Promise.all(scripts.map((script) => readFile(new URL(`dist/assets/${script}`, root), "utf8")))).join("\n");
  assert.match(bundle, /Calm screen breaks/);
  assert.match(bundle, /A break should/);
  assert.match(bundle, /Does Peeky work offline/);
  assert.match(bundle, /What Peeky can store/);
});

test("mounts reveal motion with the lazy page and fails open", async () => {
  const [entry, motion, styles] = await Promise.all([
    readFile(new URL("src/main.tsx", root), "utf8"),
    readFile(new URL("src/components/SiteMotion.tsx", root), "utf8"),
    readFile(new URL("src/styles.css", root), "utf8"),
  ]);

  assert.match(entry, /<Suspense[^>]*>[\s\S]*<SiteMotion \/>[\s\S]*<Page \/>[\s\S]*<\/Suspense>/);
  assert.match(motion, /classList\.add\("motion-ready"\)/);
  assert.match(styles, /\.motion-ready \[data-reveal\] \{ opacity: 0/);
  assert.doesNotMatch(styles, /(?:^|\n)\[data-reveal\] \{ opacity: 0/);
});
