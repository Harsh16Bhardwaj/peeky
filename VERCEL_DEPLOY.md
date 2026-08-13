# Deploy Peeky on Vercel

Peeky is a static Vite + React site. Vercel only needs to run the build and serve the generated `dist/` directory. There is no database, server process, environment variable, schema, or API required.

## Option 1: Deploy from GitHub (recommended)

1. Open [vercel.com](https://vercel.com) and sign in with GitHub.
2. Click **Add New → Project**.
3. Import the `Harsh16Bhardwaj/peeky` repository.
4. Set the **Root Directory** to `website` if the repository contains the desktop app at the parent level. If the repository itself is the website repository, leave it as `./`.
5. Confirm these project settings:

   - Framework preset: **Vite**
   - Build command: `npm run build`
   - Output directory: `dist`
   - Install command: `npm install`

6. Leave Environment Variables empty.
7. Click **Deploy**.

Vercel will build the site and publish the contents of `dist/`. The `vercel.json` file in this folder already contains the same settings and long-lived caching for the downloadable release files.

## Option 2: Deploy with the Vercel CLI

Install the CLI once, then run this from the website folder:

```powershell
npm.cmd install -g vercel
vercel login
vercel
```

For the prompts, choose the existing project or create a new one, keep the current folder as the project root, and accept the detected Vite settings. For a production deployment:

```powershell
vercel --prod
```

## Verify before deploying

From `website/`:

```powershell
npm.cmd install
npm.cmd test
```

The build creates these public routes:

- `/`
- `/download/`
- `/privacy/`
- `/downloads/Peeky-Setup-x64.exe`
- `/downloads/Peeky-Portable-x64.zip`
- `/downloads/SHA256SUMS.txt`

When you release a new desktop build, replace the files in `public/downloads/`, update `SHA256SUMS.txt`, run `npm.cmd test`, and push the changes. Vercel will redeploy automatically.

## Custom domain

In the Vercel project, open **Settings → Domains**, add your domain, and follow the DNS instructions Vercel provides. No code changes are needed.
