# Birnio Site

Astro + Starlight documentation site for `birnio.dev`, themed with Lucode Starlight.

## Commands

```sh
npm install
npm run dev
npm run build
npm run preview
```

## Cloudflare Pages

Use the Cloudflare Pages GitHub integration rather than a deploy workflow. Cloudflare
will build production deployments from `main` and preview deployments for pull
requests.

Use these settings when connecting the repository:

```text
Root directory: site
Build command: npm run build
Build output directory: dist
```

The Astro `site` URL is configured as `https://birnio.dev`.
