# Security

PromptBeacon runs locally. It does not make model API requests, upload application data, take screenshots, read prompt text, or collect telemetry.

## Reporting a vulnerability

Use [GitHub private vulnerability reporting](https://github.com/Tengify/promptbeacon/security/advisories/new) for confidential reports. Never publish exploit details, credentials, local diagnostics, or personal data in a public issue.

The current preview release is maintained. Report the version, affected platform, and a minimal reproduction using dummy data.

## Boundaries

Accessibility permission lets the application inspect selection controls in supported apps. Scans are restricted by geometry, control roles, label format, depth, queue length, node count, and time. Prompt editors and transcript text are skipped. A truncated or conflicting selection is unknown.

Settings and detection diagnostics stay in the OS local application-data directory. Diagnostics contain recognized model/effort control labels and counters. Inspect and redact them before choosing to share them. The terminal bridge reads bounded status-line input but stores only the validated session identifier, model, effort, and timestamp; other incoming fields are discarded.

The overlay cannot intercept clicks. Tauri capabilities allow event listening and unlistening only. Mutating Rust commands require the settings window. A restrictive Content Security Policy prevents remote resources and connections in the desktop UI. The landing page uses local assets and has no analytics or external requests on load; downloads and source links navigate to GitHub when selected.

Local storage is trusted to the current OS user. Unique temporary files use private permissions and atomic replacement. This does not defend against a compromised host or another process already running as that user. Keep OS and system webview updates current.

## Publishing

Build outputs, local toolchains, environment files, diagnostics, and screenshots are ignored by Git. Public examples use synthetic data only. Before a release, run the local checks and inspect the exact staged tree. Publish SHA-256 checksums with release artifacts. Preview packages currently use ad-hoc signing on macOS and have no commercial Windows signing certificate; checksums verify download integrity, not publisher identity.
