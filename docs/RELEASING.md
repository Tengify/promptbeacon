# Local releases

This project does not use GitHub Actions. Run checks and builds on your own machine.

1. Update the version in `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json`, and browser-preview version in `src/api.ts`.
2. Run `npm ci`, `npm run check:local`, and `node scripts/cargo.mjs fmt --manifest-path src-tauri/Cargo.toml --check`.
3. Build the current platform with `npm run app:build`. macOS universal builds require both Rust Apple targets: `npm run app:build -- --target universal-apple-darwin --bundles app`.
4. Sign/notarize packages using your own signing identity where available. Never commit certificates, private keys, passwords, or environment files. Ad-hoc macOS signing is suitable for explicitly labeled preview builds only.
5. Check binary strings for personal paths; release wrappers remap build paths. Use an isolated data directory and synthetic status-line payload for CLI checks. Inspect platform package contents before distribution.
6. Archive packages under ignored `artifacts/release/`; compute SHA-256 checksums. Do not commit binaries or local toolchains to Git.
7. Review `git diff --cached` and `git ls-files`; run a secret scanner against the exact staged tree. Include only source, tests with dummy data, documentation, and intended assets.
8. Generate/include third-party notices with `node scripts/licenses.mjs`, then commit and push source. Create a draft GitHub release with `gh release create`, attach packages and checksums, verify artifact names, then publish the release. Run the resulting download URLs through an HTTP check.
9. Update `website/releases.js` to the verified artifact names and run `npm run site:test`, browser smoke tests, and `npm run site:build`. Deploy `website/dist/` manually to a static host if desired.

## Optional Windows cross-build

On macOS/Linux, install cargo-xwin and LLVM tools, build frontend assets, and run `node scripts/cross-windows.mjs`. Follow cargo-xwin's SDK licensing requirements. It stages source in a temporary directory without spaces to accommodate the cross-toolchain's flag encoding. It emits `artifacts/release/PromptBeacon.exe`. Validate the GUI on a real Windows host before claiming runtime compatibility. Prefer native Windows builds for installers.

## Optional Linux x64 cross-build on an ARM host

Build `scripts/linux/Dockerfile.cross` with Docker's `linux/arm64` platform. The image includes native Rust and x64 GTK/WebKit dependencies plus a cross-linker. Run `scripts/linux/build.sh` with read-only source at `/source` and an artifact directory at `/artifacts`. This avoids emulated compiler/linker failures. The native x64 `Dockerfile` is available for x64 Linux hosts. GUI validation still needs an actual desktop session.

## Memory verification

On macOS, run `leaks --noContent --nostacks --atExit -- <Rust-test-executable> --test-threads=1`. This checks core/native ownership tests without printing allocation contents. It does not prove long-running system-webview or other-OS behavior is leak-free. Profile those on their native hosts with synthetic sessions and publish only sanitized aggregate results.
