# Barrzen Minimal Dashboard

This is the new first-class frontend for the Ferriskey admin experience, built in `dashboard/` with SvelteKit SSR.

## Goals

- match the Barrzen Minimal Design direction closely
- replace the legacy React frontend incrementally, not immediately
- keep realm-aware IAM workflows while improving layout, theming, and consistency

## Commands

```sh
pnpm install
pnpm run dev
pnpm run check
pnpm run lint
pnpm run test
pnpm run build
```

## API Client Generation

Generate a fresh OpenAPI spec from the backend first:

```sh
cd ../api
cargo run --bin ferriskey-api -- gen-api --output ../openapi.json
cd ../dashboard
```

Then generate the typed client:

```sh
pnpm run generate:api
```
