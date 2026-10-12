# PromptBeacon landing page

A static, interactive product page. The session names and model labels are dummy data. No screenshot, session history, diagnostic, or settings file is imported from a local installation.

```sh
npm run site:dev
npm run site:test
npm run site:build
```

Preview at `http://127.0.0.1:4317`. Deploy `website/dist/` to any static host manually; no GitHub Actions are needed. Client hints and user-agent fallbacks select the desktop OS. Phones/tablets show a desktop-download chooser. Every platform remains available in the download section.

Release links live in `releases.js`. Update them only after the matching packages have been built and uploaded. The page loads no third-party scripts, remote fonts, analytics, or API calls. Sora is self-hosted under its included SIL Open Font License. The logo is an original SVG from this project.
