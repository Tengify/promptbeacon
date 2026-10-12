---
name: PromptBeacon
description: A quiet local utility that makes model and effort visible before sending.
colors:
  paper: "#f7f7f2"
  ink: "#18201e"
  paper-muted: "#555e58"
  paper-line: "#d8dcd4"
  signal-blue: "#2870d5"
  signal-green: "#168577"
  signal-rust: "#c45843"
  native-canvas: "#f6f7f9"
  native-surface: "#fff"
  native-subtle: "#f0f3f6"
  native-text: "#25333e"
  native-muted: "#5b6875"
  native-line: "#dce2e8"
  native-accent: "#2b5d82"
  native-dark-canvas: "#20272e"
  native-dark-surface: "#29323b"
  native-dark-subtle: "#333e48"
  native-dark-text: "#e7edf3"
  native-dark-muted: "#b3c0cc"
  native-dark-line: "#46525f"
  native-dark-accent: "#5e91b6"
typography:
  website-display:
    fontFamily: "Sora, ui-sans-serif, system-ui, sans-serif"
    fontSize: "clamp(52px, 5.25vw, 78px)"
    fontWeight: 700
    lineHeight: 1.11
    letterSpacing: "-0.035em"
  website-headline:
    fontFamily: "Sora, ui-sans-serif, system-ui, sans-serif"
    fontSize: "clamp(32px, 3.4vw, 50px)"
    fontWeight: 700
    lineHeight: 1.18
    letterSpacing: "-0.035em"
  website-title:
    fontFamily: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "19px"
    fontWeight: 650
    lineHeight: 1.4
  website-body:
    fontFamily: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "16px"
    lineHeight: 1.55
  website-control:
    fontFamily: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "12px"
  native-headline:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "25px"
    fontWeight: 650
    lineHeight: 1.22
    letterSpacing: "-0.025em"
  native-title:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "14px"
    fontWeight: 650
  native-body:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "14px"
    lineHeight: 1.55
  native-control:
    fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "12px"
  native-mono:
    fontFamily: "ui-monospace, SFMono-Regular, Consolas, monospace"
    fontSize: "11px"
rounded:
  compact: "5px"
  control: "7px"
  action: "8px"
  container: "10px"
spacing:
  compact: "8px"
  control: "12px"
  inner: "16px"
  gutter-small: "20px"
  group: "24px"
  gutter: "30px"
components:
  website-button-primary:
    backgroundColor: "{colors.ink}"
    textColor: "#fff"
    rounded: "{rounded.action}"
    padding: "15px 22px"
  website-button-primary-hover:
    backgroundColor: "#35433c"
    textColor: "#fff"
  native-button-primary:
    backgroundColor: "{colors.native-accent}"
    textColor: "#fff"
    rounded: "{rounded.control}"
    padding: "7px 12px"
  native-button-neutral:
    backgroundColor: "{colors.native-surface}"
    textColor: "{colors.native-text}"
    rounded: "{rounded.control}"
    padding: "7px 12px"
  native-input:
    backgroundColor: "{colors.native-surface}"
    textColor: "{colors.native-text}"
    rounded: "{rounded.control}"
    padding: "6px 9px"
    height: "35px"
  native-status-chip:
    backgroundColor: "{colors.native-subtle}"
    textColor: "{colors.native-text}"
    typography: "{typography.native-control}"
    rounded: "{rounded.compact}"
    padding: "6px 9px"
  native-session-container:
    backgroundColor: "{colors.native-surface}"
    textColor: "{colors.native-text}"
    rounded: "{rounded.container}"
    padding: "14px 16px"
  website-beacon-badge:
    backgroundColor: "{colors.ink}"
    textColor: "#fff"
    rounded: "{rounded.action}"
    padding: "10px 13px"
---

# Design System: PromptBeacon

## Overview

**Creative North Star: "The Color-Calibration Bench"**

PromptBeacon is a quiet instrument beside a coding session. Its established native interface stays in operate mode: neutral surfaces, precise swatches, a large window specimen, and compact live readouts. The active selection becomes a peripheral color signal with explicit effort notation. Reading and typing take priority over decoration.

The public website presents the same mechanism at a longer viewing distance. Warm paper, bold Sora headlines, and a dark geometric coding specimen explain the utility through an explicitly synthetic rehearsal. System typography continues to carry controls and practical detail. The website introduces the instrument; it does not replace the native utility's density, typography, or OS-responsive theme.

This record is grounded in `src/style.css`, `src/main.ts`, `website/style.css`, `website/index.html`, and `website/main.js`. `PRODUCT.md` supplies the durable commitment to efficient local operation and truthful selection signals. The YAML tokens are normative; source-specific details below retain their native or website scope.

**Key Characteristics:**
- Color-calibration language: labeled swatches, window edges, and explicit effort bars.
- Native utility density alongside a spacious paper-based marketing surface.
- Self-hosted Sora for website display text; system typography for controls and native screens.
- Text equivalents and distinct state labels accompany every model signal.
- Synthetic, labeled demonstrations instead of private session captures.

## Colors

Neutral working surfaces leave blue, green, and rust recognizable as selection signals. Native control accents and model colors have separate jobs.

### Primary

- **Signal Blue** (`signal-blue`): default blue selection signal; also the website headline emphasis and keyboard focus outline.
- **Native Control Blue** (`native-accent`, `native-dark-accent`): native actions, tab selection, range controls, and focus outlines. It does not imply a particular model.

### Secondary

- **Signal Green** (`signal-green`): the second synthetic model and the website's effort anatomy specimen.

### Tertiary

- **Signal Rust** (`signal-rust`): the third synthetic model and the three-color website ribbon. It is not the native error-state color.

### Neutral

- **Warm Paper** (`paper`): website page field. **Green Ink** (`ink`) anchors text, download actions, and the specimen badge.
- **Paper Muted** and **Paper Line** (`paper-muted`, `paper-line`): supporting copy, section rules, and repeated content boundaries.
- **Native Canvas, Surface, and Subtle** (`native-canvas`, `native-surface`, `native-subtle`): utility canvas, working containers, chrome, and quiet controls.
- **Native Text, Muted, and Line** (`native-text`, `native-muted`, `native-line`): control text, explanations, and fine separators.
- Their `native-dark-*` partners follow the OS dark preference. The website retains its paper theme rather than inheriting those utility values.

Detected states use a green tonal chip, preview uses blue, and permission or connection trouble uses a warm warning chip. Labels remain the authority. The website's dark workspace and deep green local-operation section are contextual materials, not extra model categories.

**The Signal Integrity Rule.** Model colors identify a selection. They never substitute for the model name, effort label, or unknown state.

## Typography

**Display Font:** self-hosted Sora, bold only, from `website/assets/sora-bold.ttf`; the website's CSS defines its fallback stack.

**Body Font:** system sans-serif on both surfaces, with the platform-specific stacks recorded in the frontmatter.

**Label/Mono Font:** native hexadecimal labels and configuration text use the system monospace stack.

**Character:** the website's geometric display face gives short product statements presence. Native type remains compact and familiar, and all interactive controls use system typography.

### Hierarchy

- **Website Display:** the `website-display` role serves the main headline. Responsive overrides set it to 64px below the tablet breakpoint and 52px below the phone breakpoint.
- **Website Headline:** `website-headline` serves section headings; the FAQ is deliberately quieter. The small heading role uses system type rather than Sora.
- **Website Body:** `website-body` carries ordinary prose, constrained to at most 70ch. Leads are larger, while explanations and download details step down without borrowing specimen microcopy sizes.
- **Website Controls:** compact system labels serve the appearance switch and effort select. The synthetic workspace has denser local typography to reproduce a coding-window scale.
- **Native Headline and Title:** `native-headline` identifies the current settings page; `native-title` identifies utility sections.
- **Native Body and Controls:** `native-body` maintains the settings density; supporting messages and controls step down to `native-control`. Native paragraphs cap at 72ch.
- **Native Mono:** `native-mono` makes hexadecimal values and copyable configuration precise.

**The Two Working Distances Rule.** Sora belongs to website display text. Native settings, website controls, and detailed explanations keep their established system faces.

## Layout

The native settings shell is centered and capped at 1080px. Header, tabs, content, and footer share a 30px horizontal inset; it becomes 20px below 700px. Sections are separated by space and light rules. The opening view groups a current-session readout, window preview, preview controls, and appearance rows. The Model colors tab uses an editable reference table, and Connections uses explanatory rows with explicit actions. Narrow layouts wrap form groups and retain horizontal table scrolling.

The website wrapper is capped at 1320px with 56px side gutters on wide screens. Gutters become 32px below 1120px and 20px below 580px. The opening composition places the offer and action beside a larger rehearsal window. It becomes one column below 860px, keeping the working specimen and controls together. Explanation, local-operation content, setup, downloads, and FAQ use open rows and grids rather than a repeated card grid. Three-column notes and platform downloads stack at the phone breakpoint. Borders carry row structure while larger section spacing establishes the reading rhythm.

Spacing values in the frontmatter are repeated small increments, not a claim of a strict modular scale. Website section spacing is intentionally broader than native utility spacing. Responsive breakpoints and exact specimen geometry are carried in the sidecar rather than promoted to global tokens.

## Elevation & Depth

Most native and website surfaces use flat tonal layers, spacing, and fine borders. The website's dark window specimen receives a soft ambient shadow to read as an object on paper. Native preview and overlay badges receive a smaller soft shadow; the selected website appearance segment receives a subtle lift. Hard offset shadows are absent from the built system.

The overlay is visually separate without becoming a new interaction layer: a thin colored frame, optional tint, and compact corner badge carry the selection. Unknown model state adds a dashed native frame. Exact shadows, transition timings, and focus outlines are preserved in `.impeccable/design.json`.

**The Quiet Edge Rule.** The real overlay remains a peripheral, non-interactive surface. Interactive rehearsal controls belong outside the overlay.

## Shapes

Controls use gently rounded corners, with compact labels and selects tighter than actions or containers. The recurring control, action, and container sizes are recorded in the frontmatter. Circular dots identify signals; rectangular bars encode effort. Fine stroke SVG icons remain consistent with their host controls: native button icons are 17px and website icons usually 20px, both with a 1.7 stroke. Icons supplement readable labels.

Window specimens clip chrome, workspace, tint, and badge within a rounded silhouette. Native containers stay restrained; the website specimen is slightly softer and larger. A narrow three-color ribbon and a clipped underline provide website emphasis without becoming a new container treatment.

## Components

### Buttons

Native buttons are compact utility controls. The neutral variant uses the native surface and line; primary actions use the native control accent with white text. Both retain the same rounded control shape, short padding, and visible focus outline. Disabled controls reduce opacity and remove the pointer cursor. Quiet actions use a transparent background. In OS dark mode, primary actions have dedicated darker blue backgrounds.

Website primary actions are larger ink-colored download links with white text, an inline stroke icon, and generous padding. Hover lightens the ink. Secondary actions are plain text links with an icon and underline on hover. The header download action uses a fine ink border instead of a filled treatment. Native and website action heights are minimums, so labels can remain readable when wrapping.

### Chips

Native status chips are short, slightly rounded text labels. Detected, preview, permission-required, and error states change tone; unknown, waiting, and paused states remain explicitly named. A permission marker also appears on Connections when needed. State identity must not depend on color alone.

### Cards / Containers

The native current-session container is a flat surface holding a model dot, model name, state explanation, and right-aligned effort readout. Website content mostly uses open grids and ruled rows. The dark rehearsal window is a signature specimen rather than a generic content card.

### Inputs / Fields

Native fields and selects share the surface, fine line, rounded control shape, and focus outline. Model names and matching phrases become quieter borderless fields inside the reference table, revealing their boundaries on hover or focus. The color picker is paired with a readable hexadecimal value. Native errors use explicit text. The website's effort select is a compact labeled control next to the appearance switch.

### Navigation

Native tabs are borderless text controls with an accent underline for the current page. Their meaning and ordering stay Beacon, Model colors, and Connections. Website navigation uses compact text links and an outlined download action; lower-priority links are removed at narrow widths while the download action remains available. Footer links use the same restrained hover underline.

### Appearance Switch

The website's Border, Tint, and Badge controls sit in a lightly toned segmented tray. The selected segment is white with a soft shadow. Pressed state is exposed through `aria-pressed`. The controls remain outside the illustrated window, matching the real overlay's lack of interactive content.

### Window Specimen and Beacon

Native preview and live overlay share model color, explicit model text, explicit effort text, and five bars. Preview carries its own label. Unavailable effort has no filled bars; unknown model text remains visible. The overlay itself does not accept pointer input.

The website rehearsal uses synthetic Model A, Model B, and Model C sessions. Session selection updates the colored edge, badge, model name, composer label, and effort together. Appearance controls change border, tint, or badge presentation; an effort select updates that session's label and bars. The status badge announces changes politely. Labels around and inside the specimen explicitly identify it as illustrative. This local demonstration does not read a visitor's real selection.

## Do's and Don'ts

### Do:
- **Do** keep the native utility in operate mode, with Beacon, Model colors, and Connections as its settings sections.
- **Do** pair model color with a readable model name and pair effort bars with an explicit effort label.
- **Do** preserve visible distinctions between detected, preview, unknown, paused, waiting, permission-required, and error states.
- **Do** use the existing paper, system controls, and self-hosted Sora pairing for website additions.
- **Do** label illustrative sessions as synthetic and keep public examples independent of private settings, screenshots, and diagnostics.
- **Do** retain keyboard focus indicators and reduced-motion behavior.

### Don't:
- **Don't** turn the native settings window into a marketing screen or a dashboard of invented metrics.
- **Don't** guess unavailable effort, present preview as live detection, or let a stale selection look current.
- **Don't** place interactive content in the real overlay or obscure the prompt with controls.
- **Don't** use display typography for native utility controls or scale synthetic specimen microcopy into general body typography.
- **Don't** introduce external font requests, private session captures, or local-machine paths into public design examples.
