# PromptBeacon test checklist

## Launch and preview

1. Install the platform release, or run `npm run app:dev` on another supported host after installing Tauri prerequisites.
2. Click Test overlay. Confirm Preview appears, the selected model color matches, and effort changes the bar count.
3. Click controls through the overlay, type into an input, and move/resize the window. Confirm clicks and keyboard input still reach settings. The preview is tied to its initial window bounds; live detection follows target-window moves.
4. Stop preview or wait 12 seconds. It must disappear.
5. Set a different style, color, width, and position. Quit and reopen; settings must persist.
6. Close settings. Confirm the tray menu opens it again, pause/resume works, and Quit exits.

## Desktop sessions

1. Enable Accessibility on macOS. On Linux, enable AT-SPI and the target application's accessibility support. On Windows, keep PromptBeacon and the target app at the same privilege level.
2. Open a new Codex session. Compare the beacon with the model and effort shown by the app.
3. Change the model, then effort, without sending a prompt. Confirm the signal updates within one refresh interval plus the bounded scan time.
4. Switch to a resumed session with different settings. Confirm the beacon reflects that session.
5. Open a selection menu. Unknown is acceptable; an unselected menu item must never be reported as the active model.
6. Switch to an unrelated application. Confirm the beacon hides.
7. Try two visible sessions in one window. Conflicting settings must produce Unknown.
8. Repeat for Claude Code desktop. If values are not exposed, report the app version and `--probe` result. Do not treat Unknown as a successful detection. Focus the prompt field and use Connections → Copy last scan diagnostics if the signal remains unknown.
9. Try a second monitor, display scaling, fullscreen, and virtual desktops. These window-manager behaviors are not yet certified across every platform.

## Claude Code terminal bridge

1. Back up existing Claude Code settings, then merge the generated statusLine snippet from Connections.
2. Open Claude Code locally. Confirm the colored status line appears and the terminal title contains PB followed by the full session ID.
3. Confirm the beacon matches the model and actual effort label. Missing effort should show Effort unavailable.
4. Run a second session in another window/tab with a different model. Switching the active window/tab must switch the beacon when the terminal exposes that tab's title.
5. Change `/model` or `/effort` while idle. Confirm the refreshed status line and beacon agree.
6. Stop Claude Code. After 3.5 seconds without fresh events, the old session state must not remain colored as current.
7. Disable terminal title updates. The native beacon should show Unknown while the colored status line can remain useful.

## Reporting an issue

Include the OS/display system (X11 or Wayland), app versions, and expected behavior. Use dummy labels for reproduction. Inspect and redact any diagnostics before choosing to share them; screenshots and prompt content are not required.
