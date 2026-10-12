# 0.2.0 code and publication review

## Fixed findings

| Area | Problem | Resolution |
| --- | --- | --- |
| Claude effort | Labeled `Effort: Medium` picker was discarded | Anchored selection parser retains labeled effort; regression checks cover conflicts and unrelated prose |
| Worker lifecycle | Detached infinite worker retained app/detector handles | Stop flag, interruptible wait, joined exit; thread-owned detector cleanup; shutdown test |
| Native rendering | Background window calls could complicate shutdown; preview raced scans | Single coalesced main-thread update; independent preview state; explicit non-overlapping snapshot locks; state stored before rendering; hide after failure |
| Local reads | Entire corrupt files could be allocated before validation | Read limits applied before allocation/deserialization: settings/diagnostics 64 KiB, bridge events 4 KiB |
| Atomic writes | PID-based temp names collided; Windows removed settings before replace | Exclusive private unique temp files; private Unix app directories; sync and atomic replacement; concurrent-write regression |
| Bridge integrity | Stored events were not revalidated; expired files accumulated | Validate model/effort/session/freshness; bounded periodic expiration cleanup |
| Native traversal | Queue breadth and truncation could retain excess objects or partial selection | Explicit queue/depth/children caps; incomplete scans clear selection; scoped RAII references |
| macOS FFI | Copy error paths and AXValue type assumptions needed hardening | Owned copy results released on all paths; type checks before geometry conversion; stack string buffers |
| Linux lifecycle | Repeated X11 connections; stale AT-SPI bus; cross-app title match | Cached/reconnectable X11; AT-SPI reconnection; title plus app identity matching |
| Browser settings | Corrupt preview storage could bypass validation | Format/numeric/rule validation before use |
| Frontend lifecycle | Event subscriptions had no explicit disposal | Unlisten on pagehide; late subscriptions disposed; pending stale saves canceled on tray updates |
| Size | Default release profile retained unnecessary symbols/code | Size optimization, LTO, one codegen unit, symbol stripping, abort strategy, unused target features removed |
| Publication privacy | Binaries could embed absolute build paths | Release path remapping plus string checks before upload |
| Public content | Development notes/workflows unsuitable for release | MIT/security/contributing documentation; no Actions; dummy site data; staged-tree and secret scan |

## Evidence and limits

28 native Rust tests pass, including 100,000 Core Foundation ownership cycles and concurrent file replacement. macOS `leaks` reported zero leaked bytes for the full core suite. This establishes evidence for those exercised paths; it is not a proof covering every platform, provider, or long-running webview.

Frontend tests, strict Rust lint, cross-platform adapter type checks, dependency audit, landing-page OS detection tests, and responsive browser interaction checks are run locally. Full platform packages and download availability are recorded in VERIFICATION.md after release preparation.

Further native desktop testing should cover window moves/resizes, fullscreen and multiple displays, repeated session changes, permissions, tray quit, terminal tabs, and system-webview memory over long sessions. Preview packages are explicitly labeled, without a commercial signing identity. No real-machine sample, conversation, diagnostic, credential, or screenshot is part of public source or release content.
