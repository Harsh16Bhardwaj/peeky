# Peeky product site

This folder is the standalone, deployable product website for Peeky. It is intentionally separate from the Windows desktop application one directory above.

## Structure

- `app/` - website routes, shared components, metadata, and styling
- `public/product/` - approved product screenshots used by the website
- `public/downloads/` - public release artifacts and SHA-256 checksums
- `public/og.png` - social sharing artwork
- `.openai/hosting.json` - deployment configuration

The primary download points to `public/downloads/Peeky-Setup-x64.exe`. The portable ZIP and checksums are available from `/download`.

## Local development

```powershell
npm.cmd install
npm.cmd run dev
```

## Production build

```powershell
npm.cmd run build
```
