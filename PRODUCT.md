# PromptBeacon

<!-- impeccable:product-schema 1 -->

## Platform
web

Desktop utility distributed as native Tauri applications for macOS, Windows, and Linux.

## Stack
Rust + Tauri, selected by the user. A small TypeScript interface uses the system webview.

## Users
People using Codex and Claude Code who move between coding sessions and forget the selected model or reasoning effort.

## Product Purpose
Make the current model and effort recognizable before sending the next prompt.

## Operating Context
New and resumed sessions in Codex desktop, Claude Code desktop, and Claude Code terminals. Multiple sessions may share a window.

## Capabilities and Constraints
Colors identify models. Symbols or strength identify effort. The source must be the live selection, not only global defaults or past transcript data. Accessibility support and Linux compositor restrictions must be reported accurately. No screenshots or prompt content are needed.

## Brand Commitments
Name: PromptBeacon. Efficient local operation. Cross-platform support.

## Product Principles
- Read actual selected settings; never invent or silently retain stale settings.
- Keep the beacon visible without interrupting typing.
- Let users choose recognizable colors.
- Explain unsupported environments within the tool.

## Open Decisions
Assumption for the first testable build: tray utility, thin border plus label by default, optional window tint, settings saved locally, no account or cloud service.
