# PromptBeacon landing page

A static, interactive product page. The session names and model labels are dummy data. No screenshot, session history, diagnostic, or settings file is imported from a local installation.

```sh
npm run site:dev
npm run site:test
npm run site:build
```

Preview at `http://127.0.0.1:4317`. The public site is [tengify.github.io/promptbeacon](https://tengify.github.io/promptbeacon/). Client hints and user-agent fallbacks select the desktop OS. Phones/tablets show a desktop-download chooser. Every platform remains available in the download section.

Maintainers can publish updates with `npm run site:publish` after authenticating Git for the repository. This runs tests and builds locally, then pushes only the approved public assets to `gh-pages` from a temporary checkout. It preserves the branch history, refuses symlinks or unexpected files, and never force-pushes. GitHub Pages is configured to deploy from the root of that branch. `.nojekyll` skips Jekyll processing.

There is no custom GitHub Actions workflow or remote desktop build pipeline. GitHub Pages uses its own managed deployment run, which [GitHub lists as free](https://docs.github.com/en/billing/concepts/product-billing/github-actions). Other static hosts can serve `website/dist/` directly.

Release links live in `releases.js`. Update them only after the matching packages have been built and uploaded. The page loads no third-party scripts, remote fonts, analytics, or API calls. Sora is self-hosted under its included SIL Open Font License. The logo is an original SVG from this project.
