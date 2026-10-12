# Architecture

## Processes and data

The primary process is Rust/Tauri. Settings and the transparent overlay use the system webview; no Node.js runtime is bundled. One background thread inspects the focused app at the configured interval (650 ms by default, 1500 ms when paused). It publishes changed states through a single coalesced main-thread dispatch. The worker has an interruptible wait, stop signal, and join on exit; detector resources are dropped on their owner thread.

Adapters return application identity, window geometry, and a bounded set of selection controls. The parser rejects transcript/editor roles, arbitrary prose preceding model names, open menu items, and conflicting models. It identifies known Codex/Claude name patterns and actual effort labels. Color matching is separately customizable. An unrecognized model never inherits the last model's color.

macOS geometry uses logical points. Windows/X11 geometry uses physical pixels. The overlay ignores cursor input and cannot receive focus. Wayland exposes no universal third-party window-positioning API, so that adapter requests a floating badge instead of a window-sized border.

Scans stop after 420 accessible nodes or 220 ms of traversal work, and discard partial selections at that limit. IPC calls are separately bounded where the platform permits it. AX and D-Bus timeouts are short; Windows UI Automation 8 has explicit connection/transaction timeouts. These are work bounds, not a hard realtime deadline: operating-system/provider calls and scheduling can take longer.

## State machine

- starting: app boot, no inferred selection.
- inactive: no supported application is focused; overlay hidden.
- permission/error: adapter unavailable; overlay hidden and settings explain recovery.
- unknown: supported app but no trustworthy model; neutral badge/dashed border.
- partial: model read but effort unavailable/unsupported; model color plus explicit missing-effort text.
- detected: model and effort read from current controls or a fresh terminal event.
- paused: monitoring disabled; overlay hidden.
- preview: explicitly illustrative 12-second overlay on settings, stopped manually or by expiration.

State is replaced on every observation. Stored session history and global model defaults are not authoritative inputs. Unknown never silently falls back to a prior detected state.

## Terminal bridge

The same executable supports `--statusline`, consuming Claude Code's JSON from stdin. GUI initialization is skipped. Input is size-limited. Session IDs allow only alphanumeric, underscore, and hyphen characters. Model names cannot contain control characters. Events use private unique temporary files and atomic replacement, bounded reads, validated effort/model values, and a 3.5-second freshness limit. Expired event files are pruned periodically.

`--terminal-title` is opt-in and attempts to set a session marker in the terminal title (OSC title on Unix, console title on Windows). The detector only reads a fresh event when that marker exactly matches its session ID. Title updates/exposure depend on the terminal; failure leads to Unknown, not guessing.

## Boundaries

No public API is assumed for desktop apps' unsent picker selections. Desktop accessibility adapters are best-effort integrations and require runtime validation against each application's current UI. The app does not patch application bundles, inject CSS, intercept prompts, or change model choices.

Settings commands are limited to the settings window. The overlay has event-read permissions and a read-only snapshot command. Local content uses a CSP with no remote content. User-defined labels are escaped; colors and settings are validated in both TypeScript and Rust.

## Next validation

Run local native builds and the test checklist on actual Windows/Linux hosts. Add platform-specific accessible picker identifiers from observed app versions when generic composer-region detection is insufficient. A future compositor-specific Wayland integration can be added as an optional adapter without claiming universal placement.
