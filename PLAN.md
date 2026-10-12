# PromptBeacon implementation plan

Implemented: native tray utility; bounded per-platform selection adapters; click-through border/tint/badge; current model color and effort notation; editable local settings; labeled preview; optional fresh terminal bridge; local release tooling; synthetic interactive landing page.

The 0.2.0 review adds graceful worker shutdown, coalesced main-thread rendering, bounded file reads, private atomic writes, bridge-file expiration, stricter effort parsing, capped traversal queues, frontend event disposal, source-path remapping, and release size optimization.

Acceptance checks cover unknown/ambiguous states, permissions, current selection before prompting, click-through behavior, pause/resume, settings persistence, and platform limitations. Native desktop validation remains app-version and OS dependent. Follow docs/TESTING.md on each host. All builds and checks are local; the project has no GitHub Actions workflows.
