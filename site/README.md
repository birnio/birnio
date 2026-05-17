# Birnio Site

Astro + Starlight documentation site for `birnio.dev`, themed with Lucode Starlight.

## Commands

```sh
npm install
npm run dev
npm run build
npm run preview
```

## Cloudflare

Use the Cloudflare GitHub integration rather than a GitHub Actions deploy workflow.
Cloudflare will build production deployments from `main` and preview deployments
for pull requests.

Use these settings when connecting the repository:

```text
Build command: npm run build
Deploy command: npx wrangler deploy
Root directory: /site
Generated token: birnio build token
Environment variables: none
```

The deploy command reads `wrangler.jsonc`, which publishes the Astro build output
from `dist` as static assets. The Astro `site` URL is configured as
`https://birnio.dev`.
