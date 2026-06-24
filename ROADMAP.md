# lantern

## Plumbing

**Outcome:** initialize workspace with clap CLI and ratatui terminal setup

Scaffolded multi-crate workspace with present, print, init, and check subcommands, integrated
structured logging via tracing, and configured alternate screen with crossterm input handling.

## Data Model

**Outcome:** implement markdown parser with metadata and validation
Built pulldown-cmark-based parser that splits on --- separators into Vec<Slide>, supports
YAML/TOML front matter, and provides friendly error messages with file/line context.

## Rendering & Navigation

**Objective:** Build the interactive slide renderer with navigation.

| Task                      | Description                                                                       | Key Crates             |
| ------------------------- | --------------------------------------------------------------------------------- | ---------------------- |
| **✓ Ratatui Integration** | Build basic slide viewer using layout, blocks, paragraphs.                        | `ratatui`[^7]          |
| **✓ Input & State**       | Support `←/→`, `j/k`, `q`, and window resize.                                     | `crossterm`, `ratatui` |
| **✓ Status Bar**          | Display slide count, filename, clock, and theme name.                             | `ratatui`              |
| **✓ Color Styling**       | Apply consistent color palette via `owo-colors`. Define traits like `ThemeColor`. | `owo-colors`           |
| **✓ Unicode Headings**    | Use Unicode block symbols (▉▓▒░▌) for h1-h6 instead of markdown `#` syntax.       | Unicode constants      |
| **Configurable Themes**   | Base16 YAML theme system with 10 prebuilt themes.                                 | `serde_yml`, `serde`   |
|                           | Add user theme loading from config directory and CLI `--theme-file` flag.         | `dirs`                 |

## Code Highlighting via Syntect

**Objective:** Add first-class syntax highlighting using Syntect.

| Task              | Description                                                                  | Key Crates              |
| ----------------- | ---------------------------------------------------------------------------- | ----------------------- |
| **✓ Syntect**     | Load `.tmTheme` / `.sublime-syntax` definitions on startup.                  | `syntect`[^8]           |
|                   | Cache `SyntaxSet` + `ThemeSet`.                                              |                         |
| **✓ Code Blocks** | Detect fenced code blocks with language tags.                                | `syntect`, `owo-colors` |
|                   | Render syntax-highlighted text with color spans mapped to `owo-colors`.      |                         |
| **✓ Theming**     | Map terminal theme choice to Syntect theme (e.g., `"OneDark"`, `"Monokai"`). | `syntect`               |
| **✓ Performance** | Lazy-load themes and syntaxes; use `OnceLock` for caching.                   | `std::sync::OnceLock`   |
| **✓ Mode**        | Render to ANSI-colored plain text output (for `lantern print`).              | `owo-colors`            |

---

## Presenter

**Objective:** Make the live presenter reliable for real talks and fast deck editing.

| Task                     | Description                                                                            | Key Crates                       |
| ------------------------ | -------------------------------------------------------------------------------------- | -------------------------------- |
| **Finish Speaker Notes** | Parse `::: notes` blocks into `Slide.notes`; keep `Shift+N` as the notes-panel toggle. | `pulldown-cmark`, `ratatui`      |
| **Live Reload**          | Watch the source file and reload slides without restarting the presenter.              | `notify`[^9]                     |
| **Search**               | Implement `/` and `ctrl+f` search over slide text and titles; jump to the next match.  | `regex` or `fuzzy-matcher`[^10]  |
| **Navigation Polish**    | Add numeric jumps, `gg` for first slide, and `G` for last slide.                       | `crossterm`, `ratatui`           |
| **Status Metadata**      | Use parsed `author`, `date`, and `paging` metadata in the status bar.                  | internal                         |
| **Timer & Progress**     | Show session time and slide progress without adding presentation scripting yet.        | `ratatui`, `std::time`           |
| **Theme Commands**       | CLI flag `--theme <name>` switches both Syntect and lantern theme colors.              | `clap`, internal `ThemeRegistry` |

## Markdown Extension

**Objective:** Add useful markdown features without turning lantern into a plugin runtime.

| Task                       | Description                                                                      | Key Crates                    |
| -------------------------- | -------------------------------------------------------------------------------- | ----------------------------- |
| **✓ Tables & Lists**       | Render GitHub-style tables, bullets, and task lists.                             | `pulldown-cmark`, `ratatui`   |
| **✓ Horizontal Rules**     | Use box-drawing (`─`, `═`) and/or black horizontal bar (`▬`).                    | Unicode constants             |
| **✓ Admonitions**          | Render GitHub, Obsidian, and fence-style admonitions with themed borders/icons.  | `owo-colors`, internal glyphs |
| **Progressive Reveal**     | Split a slide into reveal steps with `<!-- stop -->`, reusing the normal parser. | `pulldown-cmark`              |
| **External Code Snippets** | Allow fenced code blocks to include a local file by path, with validation.       | `std::fs`                     |

## Authoring UX

**Objective:** Make the common deck-authoring loop simple before adding remote serving or arbitrary code execution.

| Task                     | Description                                                               | Key Crates           |
| ------------------------ | ------------------------------------------------------------------------- | -------------------- |
| **Implement `init`**     | Generate a small example deck with themes, code, images, and admonitions. | `include_str!`, `fs` |
| **Stdin Input**          | Support `lantern present -` and `lantern print -` for piped markdown.     | `std::io`            |
| **Custom Theme Loading** | Load a local Base16 YAML file for rendering, not just validation.         | `serde_yml`, `clap`  |
| **Config Discovery**     | Read defaults from `$XDG_CONFIG_HOME/lantern/config.toml`.                | `dirs`, `serde`      |
| **Deck Checks**          | Warn on missing image files, unknown themes, and unsupported note syntax. | internal             |

## RC

| Task                | Description                                                        | Key Crates                   |
| ------------------- | ------------------------------------------------------------------ | ---------------------------- |
| **CI/CD + Tooling** | Set up `cargo fmt`, `clippy`, `test`, and `cross` matrix CI.       | GitHub Actions               |
| **Theme Registry**  | Keep the built-in theme manifest in sync with embedded themes.     | internal                     |
| **Release**         | Tag `v1.0.0-rc.1` with changelog and binaries for major platforms. | `cargo-dist`, GitHub Actions |

## Rendering Core Extension

**Objective:** Make live, image, and video modes all run on the same slide/timeline + frame renderer pipeline.

| Task                      | Description                                                                                         | Key Crates                                 |
| ------------------------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| **Event Timeline Core**   | Compile slides into an `Event` timeline (show slide, type, run command, wait, transition, capture). | internal `timeline` module                 |
| **Virtual Terminal Core** | Implement PTY + ANSI parser → `TerminalBuffer { cells, colors, attrs }` shared by live/video/image. | `portable-pty` (or similar), internal ANSI |
| **Frame Layout Engine**   | Map title/body/terminal regions into a logical canvas (cells or pixels) for all renderers.          | internal `layout` module                   |
| **Renderer Trait**        | Define `Renderer` trait (`begin`, `handle_event`, `end`) with impls for Live, Image, and Video.     | internal `renderer` module                 |

## Export: Images

**Objective:** Generate high-quality PNG/SVG snapshots of any slide (Freeze-style) directly from the slide + layout + terminal state.

| Task                          | Description                                                                                               | Key Crates                        |
| ----------------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------- |
| **Canvas → Pixmap**           | Implement a `FrameRasterizer` that turns a `Frame` + layout into an RGBA pixmap (background, panes, etc). | `tiny-skia`                       |
| **Text Rendering**            | Render slide titles/body text via glyph rasterization and simple layout (left/center, line wrapping).     | `ab_glyph`                        |
| **Terminal Snapshot Mode**    | Convert `TerminalBuffer` into a rendered terminal "window" (frame, tabs, padding, cursor).                | `tiny-skia`, `ab_glyph`           |
| **Slide Screenshot CLI**      | `lantern export-image deck.md --slide 5 --output slide-5.png` (PNG by default, optional SVG/WebP).        | `clap`, `image`                   |
| **Batch Export**              | `--all` / `--range 3..7` to dump multiple slides, naming convention like `deck-003.png`.                  | `image`                           |
| **Deterministic Layout Test** | Golden tests comparing generated PNGs against fixtures for regression in layout and text.                 | `image`, integration test harness |

## Export: Video

**Objective:** Produce MP4/WebM/GIF recordings of a scripted terminal+slides run (VHS-style) directly from the markdown deck.

| Task                          | Description                                                                                                    | Key Crates                                              |                   |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ----------------- |
| **Timeline Scheduling**       | Extend `Event` to carry timestamps or durations; implement `Scheduler` to emit frames at target FPS.           | internal `timeline` module                              |                   |
| **Frame Capture Loop**        | Drive the same layout/rasterizer used for images at N FPS, yielding a sequence of RGBA frames.                 | `tiny-skia`, `image`                                    |                   |
| **FFmpeg Binding Layer**      | Wrap `ffmpeg-next` to open an encoder, configure codec/container, and accept raw frames.                       | `ffmpeg-next`                                           |                   |
| **Video Export CLI**          | `lantern export-video deck.md --output demo.mp4 --fps 30 --duration 120s` (or auto-duration from events).      | `clap`, internal encoder                                |                   |
| **GIF / WebM Variants**       | Add `--format gif                                                                                              | webm mapping to appropriate ffmpeg muxer/codec presets. | `ffmpeg-next`[^7] |
| **Typing & Cursor Effects**   | Represent typing, deletes, cursor blinks as timeline events, so video export matches live presentation feel.   | internal `timeline`, terminal core                      |                   |
| **Audio-less Simplification** | Keep V1 video export silent (no audio tracks) for simpler ffmpeg integration and smaller binaries.             | `ffmpeg-next`                                           |                   |
| **Performance Tuning**        | Measure memory/CPU for long decks; stream frames to ffmpeg (no full buffering) and expose `--quality` presets. | `ffmpeg-next`, `image`                                  |                   |

## Export: Social Media

**Objective:** Generate vertical (portrait) slides optimized for short-form vertical video.

| Task                       | Description                                                                                      | Key Crates                    |
| -------------------------- | ------------------------------------------------------------------------------------------------ | ----------------------------- |
| **Portrait Layout Engine** | Implement 9:16 aspect ratio layout with vertical constraints (1080x1920, 720x1280).              | internal `layout` module      |
| **Mobile-Optimized Text**  | Larger font sizes, reduced content density, and simplified layouts for mobile readability.       | `ab_glyph`, `tiny-skia`       |
| **Vertical Export CLI**    | `lantern export-vertical deck.md --output reel.mp4` with preset dimensions for each platform.    | `clap`, internal encoder      |
| **Platform Presets**       | Built-in presets: `instagram-reel`, `tiktok`, `youtube-shorts` with optimal resolution/duration. | internal preset registry      |
| **Content Adaptation**     | Auto-scale or warn when horizontal content doesn't fit portrait orientation.                     | internal `layout` module      |
| **Safe Zones**             | Respect platform UI overlays (captions, profile pics) with configurable safe zones.              | internal `layout` module      |
| **Swipe Animations**       | Optional slide transition effects optimized for vertical scrolling behavior.                     | internal `timeline`, `ffmpeg` |

## Authoring & UX for Export

**Objective:** Make "slides → image/video" a natural extension of your current CLI and authoring workflow.

| Task                     | Description                                                                                      | Key Crates                   |
| ------------------------ | ------------------------------------------------------------------------------------------------ | ---------------------------- |
| **Export Subcommands**   | Add `lantern export-image` and `lantern export-video` commands with shared flags (theme, range). | `clap`                       |
| **Frontmatter Controls** | Support per-deck/per-slide frontmatter: `fps`, `default_duration`, `transition`, `record: true`. | `pulldown-cmark-frontmatter` |
| **Deterministic Seeds**  | Add `--seed` for any animations (typing jitter, cursor blink timing) to keep exports repeatable. | internal `timeline`          |
| **Preset Profiles**      | Presets like `social-card`, `doc-screenshot`, `talk-demo` mapping to resolution + theme.         | internal profile registry    |

[^7]: <https://docs.rs/ratatui/latest/ratatui/>

[^8]: <https://docs.rs/syntect/latest/syntect/>

[^9]: <https://docs.rs/notify/latest/notify/>

[^10]: <https://docs.rs/fuzzy-matcher>
