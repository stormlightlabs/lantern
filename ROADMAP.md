# lantern

This roadmap tracks the larger plans for lantern. Actionable checkbox tasks live in
[todo.md](todo.md), while implementation context and crate notes stay here.

## Rendering & Navigation

**Objective:** Build the interactive slide renderer with navigation.

Tasks: see [todo.md](todo.md#rendering--navigation).

**Crate notes:**

- Ratatui Integration: `ratatui`[^7].
- Input & State: `crossterm`, `ratatui`.
- Status Bar: `ratatui`.
- Color Styling: `owo-colors`.
- Unicode Headings: Unicode constants.
- Configurable Themes: `serde_yml`, `serde`; user theme loading uses `dirs`.

## Code Highlighting via Syntect

**Objective:** Add first-class syntax highlighting using Syntect.

Tasks: see [todo.md](todo.md#code-highlighting-via-syntect).

**Crate notes:**

- Syntect: `syntect`[^8].
- Code Blocks: `syntect`, `owo-colors`.
- Theming: `syntect`.
- Performance: `std::sync::OnceLock`.
- Mode: `owo-colors`.

## Presenter

**Objective:** Make the live presenter reliable for real talks and fast deck editing.

Tasks: see [todo.md](todo.md#presenter).

**Crate notes:**

- Finish Speaker Notes: `pulldown-cmark`, `ratatui`.
- Live Reload: `notify`[^9].
- Search: `regex` or `fuzzy-matcher`[^10].
- Navigation Polish: `crossterm`, `ratatui`.
- Status Metadata: internal.
- Timer & Progress: `ratatui`, `std::time`.
- Theme Commands: `clap`, internal `ThemeRegistry`.

## Markdown Extension

**Objective:** Add useful markdown features without turning lantern into a plugin runtime.

Tasks: see [todo.md](todo.md#markdown-extension).

**Crate notes:**

- Tables & Lists: `pulldown-cmark`, `ratatui`.
- Horizontal Rules: Unicode constants.
- Admonitions: `owo-colors`, internal glyphs.
- Progressive Reveal: `pulldown-cmark`.
- External Code Snippets: `std::fs`.

## Authoring UX

**Objective:** Make the common deck-authoring loop simple before adding remote serving or arbitrary code execution.

Tasks: see [todo.md](todo.md#authoring-ux).

**Crate notes:**

- Implement `init`: `include_str!`, `fs`.
- Stdin Input: `std::io`.
- Custom Theme Loading: `serde_yml`, `clap`.
- Config Discovery: `dirs`, `serde`.
- Deck Checks: internal.

## RC

Tasks: see [todo.md](todo.md#rc).

**Crate notes:**

- CI/CD + Tooling: GitHub Actions.
- Theme Registry: internal.
- Release: `cargo-dist`, GitHub Actions.

## Rendering Core Extension

**Objective:** Make live, image, and video modes all run on the same slide/timeline + frame renderer pipeline.

Tasks: see [todo.md](todo.md#rendering-core-extension).

**Crate notes:**

- Event Timeline Core: internal `timeline` module.
- Virtual Terminal Core: `portable-pty` (or similar), internal ANSI.
- Frame Layout Engine: internal `layout` module.
- Renderer Trait: internal `renderer` module.

## Export: Images

**Objective:** Generate high-quality PNG/SVG snapshots of any slide (Freeze-style) directly from the slide + layout + terminal state.

Tasks: see [todo.md](todo.md#export-images).

**Crate notes:**

- Canvas -> Pixmap: `tiny-skia`.
- Text Rendering: `ab_glyph`.
- Terminal Snapshot Mode: `tiny-skia`, `ab_glyph`.
- Slide Screenshot CLI: `clap`, `image`.
- Batch Export: `image`.
- Deterministic Layout Test: `image`, integration test harness.

## Export: Video

**Objective:** Produce MP4/WebM/GIF recordings of a scripted terminal+slides run (VHS-style) directly from the markdown deck.

Tasks: see [todo.md](todo.md#export-video).

**Crate notes:**

- Timeline Scheduling: internal `timeline` module.
- Frame Capture Loop: `tiny-skia`, `image`.
- FFmpeg Binding Layer: `ffmpeg-next`.
- Video Export CLI: `clap`, internal encoder.
- GIF / WebM Variants: `ffmpeg-next`.
- Typing & Cursor Effects: internal `timeline`, terminal core.
- Audio-less Simplification: `ffmpeg-next`.
- Performance Tuning: `ffmpeg-next`, `image`.

## Export: Social Media

**Objective:** Generate vertical (portrait) slides optimized for short-form vertical video.

Tasks: see [todo.md](todo.md#export-social-media).

**Crate notes:**

- Portrait Layout Engine: internal `layout` module.
- Mobile-Optimized Text: `ab_glyph`, `tiny-skia`.
- Vertical Export CLI: `clap`, internal encoder.
- Platform Presets: internal preset registry.
- Content Adaptation: internal `layout` module.
- Safe Zones: internal `layout` module.
- Swipe Animations: internal `timeline`, `ffmpeg`.

## Authoring & UX for Export

**Objective:** Make "slides -> image/video" a natural extension of your current CLI and authoring workflow.

Tasks: see [todo.md](todo.md#authoring--ux-for-export).

**Crate notes:**

- Export Subcommands: `clap`.
- Frontmatter Controls: `pulldown-cmark-frontmatter`.
- Deterministic Seeds: internal `timeline`.
- Preset Profiles: internal profile registry.

[^7]: <https://docs.rs/ratatui/latest/ratatui/>

[^8]: <https://docs.rs/syntect/latest/syntect/>

[^9]: <https://docs.rs/notify/latest/notify/>

[^10]: <https://docs.rs/fuzzy-matcher>
