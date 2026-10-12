# Contributing

Install Node.js 22+, Rust 1.99+, and [Tauri's platform prerequisites](https://tauri.app/start/prerequisites/). Then run:

```sh
npm ci
npm run app:dev
```

Before opening a pull request:

```sh
npm run check:local
node scripts/cargo.mjs fmt --manifest-path src-tauri/Cargo.toml --check
npm run site:build
```

All builds and checks run locally. This repository contains no GitHub Actions workflows.

For a detection change, add a regression test with dummy labels and validate against the relevant native OS. Unknown is preferable to showing a stale or ambiguous selection. Do not read prompt fields or use chat history as a fallback. Raw native references and thread-affine COM objects must be released on every path.

Never commit credentials, application settings, status-line events, personal paths, diagnostics, or screenshots of real sessions. Use synthetic data for examples, tests, and website demonstrations. See [Security](SECURITY.md) for private vulnerability reports and [local release instructions](docs/RELEASING.md) for packaging.
