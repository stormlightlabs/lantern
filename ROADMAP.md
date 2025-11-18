# lantern

## Plumbing

__Outcome:__ initialize workspace with clap CLI and ratatui terminal setup

Scaffolded multi-crate workspace with present, print, init, and check subcommands, integrated structured logging via tracing, and configured alternate screen with crossterm input handling.

## Data Model

__Outcome:__  implement markdown parser with metadata and validation
Built pulldown-cmark-based parser that splits on --- separators into Vec<Slide>, supports YAML/TOML front matter, and provides friendly error messages with file/line context.

## Rendering & Navigation

__Objective:__ Build the interactive slide renderer with navigation.

| Task                      | Description                                                                                       | Key Crates             |
| ------------------------- | ------------------------------------------------------------------------------------------------- | ---------------------- |
| __✓ Ratatui Integration__ | Build basic slide viewer using layout, blocks, paragraphs.                                        | `ratatui`[^7]          |
| __✓ Input & State__       | Support `←/→`, `j/k`, `q`, numeric jumps, and window resize.                                      | `crossterm`, `ratatui` |
| __✓ Status Bar__          | Display slide count, filename, clock, and theme name.                                             | `ratatui`              |
| __✓ Color Styling__       | Apply consistent color palette via `owo-colors`. Define traits like `ThemeColor`.                 | `owo-colors`           |
| __✓ Unicode Headings__    | Use Unicode block symbols (▉▓▒░▌) for h1-h6 instead of markdown `#` syntax.                       | Unicode constants      |
| __Configurable Themes__   | Base16 YAML theme system with 10 prebuilt themes.                                                 | `serde_yml`, `serde`   |
|                           | Add user theme loading from config directory and CLI `--theme-file` flag.                         | `dirs`                 |

## Code Highlighting via Syntect

__Objective:__ Add first-class syntax highlighting using Syntect.

| Task                | Description                                                                  | Key Crates              |
| ------------------- | ---------------------------------------------------------------------------- | ----------------------- |
| __✓ Syntect__       | Load `.tmTheme` / `.sublime-syntax` definitions on startup.                  | `syntect`[^8]           |
|                     | Cache `SyntaxSet` + `ThemeSet`.                                              |                         |
| __✓ Code Blocks__   | Detect fenced code blocks with language tags.                                | `syntect`, `owo-colors` |
|                     | Render syntax-highlighted text with color spans mapped to `owo-colors`.      |                         |
| __✓ Theming__       | Map terminal theme choice to Syntect theme (e.g., `"OneDark"`, `"Monokai"`). | `syntect`               |
| __✓ Performance__   | Lazy-load themes and syntaxes; use `OnceLock` for caching.                   | `std::sync::OnceLock`   |
| __✓ Mode__          | Render to ANSI-colored plain text output (for `lantern print`).              | `owo-colors`            |

---

## Presenter

__Objective:__ Introduce features for live presentations and authoring convenience.

| Task                 | Description                                                   | Key Crates                       |
| -------------------- | ------------------------------------------------------------- | -------------------------------- |
| __Speaker Notes__    | `N` toggles speaker notes (parsed via `::: notes`).           | `ratatui`                        |
|                      | Note: `n` & `p` move forward & backwards                      |                                  |
| __Timer & Progress__ | Session timer + per-slide progress bar.                       | `ratatui`, `chrono`              |
| __Live Reload__      | File watcher auto-refreshes content.                          | `notify`[^9]                     |
| __Search__           | Fuzzy find slide titles via `ctrl+f`.                         | `fuzzy-matcher`[^10]             |
| __Theme Commands__   | CLI flag `--theme <name>` switches both Syntect + owo themes. | `clap`, internal `ThemeRegistry` |

## Markdown Extension

__Objective:__ Add richness and visual polish to text and layout.

| Task                   | Description                                                   | Key Crates                    |
| ---------------------- | ------------------------------------------------------------- | ----------------------------- |
| __✓ Tables & Lists__   | Render GitHub-style tables, bullets, and task lists           | `pulldown-cmark`, `ratatui`   |
| __✓ Horizontal Rules__ | Use box-drawing (`─`, `═`) and/or black horizontal bar (`▬`)  | Unicode constants             |
| __Admonitions__        | Highlighted boxes with icons (use `:::` directives)           | `owo-colors`, internal glyphs |
|                        | Support obsidian & GH admonitions                             |                               |
| __Generators__         | `lantern init` scaffolds an example deck with code and notes  | `include_str!`, `fs`          |

## RC

| Task                 | Description                                                        | Key Crates                   |
| -------------------- | ------------------------------------------------------------------ | ---------------------------- |
| __CI/CD + Tooling__  | Setup `cargo fmt`, `clippy`, `test`, and `cross` matrix CI         | GitHub Actions               |
| __Config Discovery__ | Read from `$XDG_CONFIG_HOME/lantern/config.toml` for defaults      | `dirs`, `serde`              |
| __Theme Registry__   | Built-in theme manifest (e.g., `onedark`, `solarized`, `plain`).   | Internal                     |
| __Release__          | Tag `v1.0.0-rc.1` with changelog and binaries for major platforms. | `cargo-dist`, GitHub Actions |

## Rendering Core Extension

__Objective:__ Make live, image, and video modes all run on the same slide/timeline + frame renderer pipeline.

| Task                      | Description                                                                                         | Key Crates                                 |
| ------------------------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| __Event Timeline Core__   | Compile slides into an `Event` timeline (show slide, type, run command, wait, transition, capture). | internal `timeline` module                 |
| __Virtual Terminal Core__ | Implement PTY + ANSI parser → `TerminalBuffer { cells, colors, attrs }` shared by live/video/image. | `portable-pty` (or similar), internal ANSI |
| __Frame Layout Engine__   | Map title/body/terminal regions into a logical canvas (cells or pixels) for all renderers.          | internal `layout` module                   |
| __Renderer Trait__        | Define `Renderer` trait (`begin`, `handle_event`, `end`) with impls for Live, Image, and Video.     | internal `renderer` module                 |

## Export: Images

__Objective:__ Generate high-quality PNG/SVG snapshots of any slide (Freeze-style) directly from the slide + layout + terminal state.

| Task                          | Description                                                                                               | Key Crates                        |
| ----------------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------- |
| __Canvas → Pixmap__           | Implement a `FrameRasterizer` that turns a `Frame` + layout into an RGBA pixmap (background, panes, etc). | `tiny-skia`                       |
| __Text Rendering__            | Render slide titles/body text via glyph rasterization and simple layout (left/center, line wrapping).     | `ab_glyph`                        |
| __Terminal Snapshot Mode__    | Convert `TerminalBuffer` into a rendered terminal "window" (frame, tabs, padding, cursor).                | `tiny-skia`, `ab_glyph`           |
| __Slide Screenshot CLI__      | `lantern export-image deck.md --slide 5 --output slide-5.png` (PNG by default, optional SVG/WebP).        | `clap`, `image`                   |
| __Batch Export__              | `--all` / `--range 3..7` to dump multiple slides, naming convention like `deck-003.png`.                  | `image`                           |
| __Deterministic Layout Test__ | Golden tests comparing generated PNGs against fixtures for regression in layout and text.                 | `image`, integration test harness |

## Export: Video

__Objective:__ Produce MP4/WebM/GIF recordings of a scripted terminal+slides run (VHS-style) directly from the markdown deck.

| Task                          | Description                                                                                                    | Key Crates                                               |                   |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ----------------- |
| __Timeline Scheduling__       | Extend `Event` to carry timestamps or durations; implement `Scheduler` to emit frames at target FPS.           | internal `timeline` module                               |                   |
| __Frame Capture Loop__        | Drive the same layout/rasterizer used for images at N FPS, yielding a sequence of RGBA frames.                 | `tiny-skia`, `image`                                     |                   |
| __FFmpeg Binding Layer__      | Wrap `ffmpeg-next` to open an encoder, configure codec/container, and accept raw frames.                       | `ffmpeg-next`                                            |                   |
| __Video Export CLI__          | `lantern export-video deck.md --output demo.mp4 --fps 30 --duration 120s` (or auto-duration from events).      | `clap`, internal encoder                                 |                   |
| __GIF / WebM Variants__       | Add `--format gif                                                                                              | webm mapping to appropriate ffmpeg muxer/codec presets.  | `ffmpeg-next`[^7] |
| __Typing & Cursor Effects__   | Represent typing, deletes, cursor blinks as timeline events, so video export matches live presentation feel.   | internal `timeline`, terminal core                       |                   |
| __Audio-less Simplification__ | Keep V1 video export silent (no audio tracks) for simpler ffmpeg integration and smaller binaries.             | `ffmpeg-next`                                            |                   |
| __Performance Tuning__        | Measure memory/CPU for long decks; stream frames to ffmpeg (no full buffering) and expose `--quality` presets. | `ffmpeg-next`, `image`                                   |                   |

## Export: Social Media

__Objective:__ Generate vertical (portrait) slides optimized for short-form vertical video.

| Task                       | Description                                                                                      | Key Crates                    |
| -------------------------- | ------------------------------------------------------------------------------------------------ | ----------------------------- |
| __Portrait Layout Engine__ | Implement 9:16 aspect ratio layout with vertical constraints (1080x1920, 720x1280).              | internal `layout` module      |
| __Mobile-Optimized Text__  | Larger font sizes, reduced content density, and simplified layouts for mobile readability.       | `ab_glyph`, `tiny-skia`       |
| __Vertical Export CLI__    | `lantern export-vertical deck.md --output reel.mp4` with preset dimensions for each platform.    | `clap`, internal encoder      |
| __Platform Presets__       | Built-in presets: `instagram-reel`, `tiktok`, `youtube-shorts` with optimal resolution/duration. | internal preset registry      |
| __Content Adaptation__     | Auto-scale or warn when horizontal content doesn't fit portrait orientation.                     | internal `layout` module      |
| __Safe Zones__             | Respect platform UI overlays (captions, profile pics) with configurable safe zones.              | internal `layout` module      |
| __Swipe Animations__       | Optional slide transition effects optimized for vertical scrolling behavior.                     | internal `timeline`, `ffmpeg` |

## Authoring & UX for Export

__Objective:__ Make "slides → image/video" a natural extension of your current CLI and authoring workflow.

| Task                     | Description                                                                                      | Key Crates                   |
| ------------------------ | ------------------------------------------------------------------------------------------------ | ---------------------------- |
| __Export Subcommands__   | Add `lantern export-image` and `lantern export-video` commands with shared flags (theme, range). | `clap`                       |
| __Frontmatter Controls__ | Support per-deck/per-slide frontmatter: `fps`, `default_duration`, `transition`, `record: true`. | `pulldown-cmark-frontmatter` |
| __Deterministic Seeds__  | Add `--seed` for any animations (typing jitter, cursor blink timing) to keep exports repeatable. | internal `timeline`          |
| __Preset Profiles__      | Presets like `social-card`, `doc-screenshot`, `talk-demo` mapping to resolution + theme.         | internal profile registry    |

[^7]: <https://docs.rs/ratatui/latest/ratatui/>
[^8]: <https://docs.rs/syntect/latest/syntect/>
[^9]: <https://docs.rs/notify/latest/notify/>
[^10]: <https://docs.rs/fuzzy-matcher>
