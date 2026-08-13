# Peeky product site

A plain static Vite + React website for Peeky. It has no database, schema, server runtime, authentication, cloud SDK, or hosting-provider integration.

## Folder structure

- `src/` - site pages, shared components, and styling
- `download/index.html` - standalone download-page entry
- `privacy/index.html` - standalone privacy-page entry
- `public/product/` - approved product screenshots
- `public/downloads/` - installer, portable ZIP, and SHA-256 checksums
- `public/og.png` - social sharing image
- `dist/` - complete self-hostable output created by the build

## Run locally

```powershell
npm.cmd install
npm.cmd run dev
```

## Build for self-hosting

```powershell
npm.cmd run build
```

Upload the **contents** of `dist/` to the public/root folder of any static host: Apache, Nginx, Cloudflare Pages, GitHub Pages, Netlify, Vercel static hosting, S3, or a basic cPanel host. No Node process is needed after the build.

The primary installer URL is `/downloads/Peeky-Setup-x64.exe`. Replace the files in `public/downloads/` and rebuild whenever shipping a new desktop release.

## Deploy on Vercel

See [VERCEL_DEPLOY.md](./VERCEL_DEPLOY.md) for the GitHub and CLI deployment steps.
