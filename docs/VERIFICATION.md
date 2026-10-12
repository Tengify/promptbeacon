# 0.2.0 verification

All checks and packages were produced locally. This repository has no GitHub Actions workflows.

## Code and memory

- TypeScript production build; 8 frontend behavior tests; 3 landing-page OS/download tests.
- 28 Rust tests, including Claude's labeled effort picker, conflict/stale-state handling, configuration validation, concurrent atomic writes, bounded file reads, and interruptible worker shutdown.
- Strict Rust Clippy and formatting checks.
- Windows x64 and Linux adapter type checks.
- macOS native `leaks` over the full Rust suite: zero leaks / zero leaked bytes. Includes 100,000 dummy Core Foundation ownership cycles. Allocation content was suppressed.
- Frontend dependency audit: zero reported vulnerabilities at review time.

The leak result covers the exercised core/native paths. Long-running system-webview memory, platform accessibility providers, and Windows/Linux GUI memory still require native-host profiling. No claim of universal leak freedom is made.

## Packages

- macOS universal app, Apple Silicon + Intel: 8.47 MiB app bundle (8.42 MiB universal executable). Apple Silicon executable: approximately 3.99 MiB, down from 11.89 MiB before release optimization (about 66% smaller). Universal builds include a second architecture.
- Windows x64 portable executable: approximately 4.10 MiB; built with the local MSVC cross-toolchain.
- Linux x64 executable: approximately 5.67 MiB; built with a native ARM compiler and x64 sysroot, packaged as a Debian installer with runtime dependencies.
- macOS preview bundle ad-hoc signed and deep/strict signature verification passed.
- Packaged macOS and Linux status-line modes tested with dummy JSON: stores only four allowed event fields and rejects invalid session identifiers.
- Binary string checks find zero personal username/project-path matches in all three platform artifacts. Build/source paths are remapped.
- Downloads: macOS universal ZIP 3.76 MiB, Windows portable ZIP 1.75 MiB, Linux Debian installer 1.73 MiB. SHA-256 checksums accompany them.

Build success is not native GUI certification. Windows/Linux desktop runtime behavior, Intel macOS runtime, fullscreen, multiple displays, terminal-tab binding, and different app versions remain native-host validation tasks. Preview builds have no commercial signing certificate/notarization.

## Landing page and publication privacy

Responsive browser checks passed at 1440, 390, and 320 pixels: no horizontal overflow, working session/style/effort controls, OS-specific download labels, mobile/tablet chooser, and zero page errors. Reduced-motion behavior is supported. Self-hosted assets and a restrictive CSP avoid third-party requests on load.

Public examples are synthetic. The exact staged source was checked for personal paths, credentials, local data, screenshots, and workflows, then scanned with Gitleaks with redacted output. Local toolchains, generated schemas, artifacts, diagnostics, and test captures are ignored. No real session or local-machine sample is part of public source or packages.

Review findings and fixes are recorded in REVIEW.md. Release artifacts are verified through their published GitHub download URLs after upload.
