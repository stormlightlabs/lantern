# lantern todo

Tasks moved from [ROADMAP.md](ROADMAP.md).

## Rendering & Navigation

- [x] **Ratatui Integration**: Build basic slide viewer using layout, blocks, paragraphs.
- [x] **Input & State**: Support `←/→`, `j/k`, `q`, and window resize.
- [x] **Status Bar**: Display slide count, filename, clock, and theme name.
- [x] **Color Styling**: Apply consistent color palette via `owo-colors`. Define traits like `ThemeColor`.
- [x] **Unicode Headings**: Use Unicode block symbols (▉▓▒░▌) for h1-h6 instead of markdown `#` syntax.
- [ ] **Configurable Themes**: Base16 YAML theme system with 10 prebuilt themes.
- [ ] **Configurable Themes**: Add user theme loading from config directory and CLI `--theme-file` flag.

## Code Highlighting via Syntect

- [x] **Syntect**: Load `.tmTheme` / `.sublime-syntax` definitions on startup.
- [x] **Syntect**: Cache `SyntaxSet` + `ThemeSet`.
- [x] **Code Blocks**: Detect fenced code blocks with language tags.
- [x] **Code Blocks**: Render syntax-highlighted text with color spans mapped to `owo-colors`.
- [x] **Theming**: Map terminal theme choice to Syntect theme (e.g., `"OneDark"`, `"Monokai"`).
- [x] **Performance**: Lazy-load themes and syntaxes; use `OnceLock` for caching.
- [x] **Mode**: Render to ANSI-colored plain text output (for `lantern print`).
  - [ ] This should support `NO_COLOR`

## Presenter

- [x] **Finish Speaker Notes**: Parse `::: notes` blocks into `Slide.notes`; keep `Shift+N` as the notes-panel toggle.
- [x] **Live Reload**: Watch the source file and reload slides without restarting the presenter.
- [ ] **Search**: Implement `/` and `ctrl+f` search over slide text and titles; jump to the next match.
- [ ] **Navigation Polish**: Add numeric jumps, `gg` for first slide, and `G` for last slide.
- [ ] **Status Metadata**: Use parsed `author`, `date`, and `paging` metadata in the status bar.
- [ ] **Timer & Progress**: Show session time and slide progress without adding presentation scripting yet.
- [ ] **Theme Commands**: CLI flag `--theme <name>` switches both Syntect and lantern theme colors.

## Markdown Extension

- [x] **Tables & Lists**: Render GitHub-style tables, bullets, and task lists.
- [x] **Horizontal Rules**: Use box-drawing (`─`, `═`) and/or black horizontal bar (`▬`).
- [x] **Admonitions**: Render GitHub, Obsidian, and fence-style admonitions with themed borders/icons.
- [ ] **Progressive Reveal**: Split a slide into reveal steps with `<!-- stop -->`, reusing the normal parser.
- [ ] **External Code Snippets**: Allow fenced code blocks to include a local file by path, with validation.

## Authoring UX

- [ ] **Implement `init`**: Generate a small example deck with themes, code, images, and admonitions.
- [ ] **Stdin Input**: Support `lantern present -` and `lantern print -` for piped markdown.
- [ ] **Custom Theme Loading**: Load a local Base16 YAML file for rendering, not just validation.
- [ ] **Config Discovery**: Read defaults from `$XDG_CONFIG_HOME/lantern/config.toml`.
- [ ] **Deck Checks**: Warn on missing image files, unknown themes, and unsupported note syntax.

## RC

- [ ] **CI/CD + Tooling**: Set up `cargo fmt`, `clippy`, `test`, and `cross` matrix CI.
- [ ] **Theme Registry**: Keep the built-in theme manifest in sync with embedded themes.
- [ ] **Release**: Tag `v1.0.0-rc.1` with changelog and binaries for major platforms.

## Rendering Core Extension

- [ ] **Event Timeline Core**: Compile slides into an `Event` timeline (show slide, type, run command, wait, transition, capture).
- [ ] **Virtual Terminal Core**: Implement PTY + ANSI parser -> `TerminalBuffer { cells, colors, attrs }` shared by live/video/image.
- [ ] **Frame Layout Engine**: Map title/body/terminal regions into a logical canvas (cells or pixels) for all renderers.
- [ ] **Renderer Trait**: Define `Renderer` trait (`begin`, `handle_event`, `end`) with impls for Live, Image, and Video.

## Export: Images

- [ ] **Canvas -> Pixmap**: Implement a `FrameRasterizer` that turns a `Frame` + layout into an RGBA pixmap (background, panes, etc).
- [ ] **Text Rendering**: Render slide titles/body text via glyph rasterization and simple layout (left/center, line wrapping).
- [ ] **Terminal Snapshot Mode**: Convert `TerminalBuffer` into a rendered terminal "window" (frame, tabs, padding, cursor).
- [ ] **Slide Screenshot CLI**: `lantern export-image deck.md --slide 5 --output slide-5.png` (PNG by default, optional SVG/WebP).
- [ ] **Batch Export**: `--all` / `--range 3..7` to dump multiple slides, naming convention like `deck-003.png`.
- [ ] **Deterministic Layout Test**: Golden tests comparing generated PNGs against fixtures for regression in layout and text.

## Export: Video

- [ ] **Timeline Scheduling**: Extend `Event` to carry timestamps or durations; implement `Scheduler` to emit frames at target FPS.
- [ ] **Frame Capture Loop**: Drive the same layout/rasterizer used for images at N FPS, yielding a sequence of RGBA frames.
- [ ] **FFmpeg Binding Layer**: Wrap `ffmpeg-next` to open an encoder, configure codec/container, and accept raw frames.
- [ ] **Video Export CLI**: `lantern export-video deck.md --output demo.mp4 --fps 30 --duration 120s` (or auto-duration from events).
- [ ] **GIF / WebM Variants**: Add `--format gif | webm` mapping to appropriate ffmpeg muxer/codec presets.
- [ ] **Typing & Cursor Effects**: Represent typing, deletes, cursor blinks as timeline events, so video export matches live presentation feel.
- [ ] **Audio-less Simplification**: Keep V1 video export silent (no audio tracks) for simpler ffmpeg integration and smaller binaries.
- [ ] **Performance Tuning**: Measure memory/CPU for long decks; stream frames to ffmpeg (no full buffering) and expose `--quality` presets.

## Export: Social Media

- [ ] **Portrait Layout Engine**: Implement 9:16 aspect ratio layout with vertical constraints (1080x1920, 720x1280).
- [ ] **Mobile-Optimized Text**: Larger font sizes, reduced content density, and simplified layouts for mobile readability.
- [ ] **Vertical Export CLI**: `lantern export-vertical deck.md --output reel.mp4` with preset dimensions for each platform.
- [ ] **Platform Presets**: Built-in presets: `instagram-reel`, `tiktok`, `youtube-shorts` with optimal resolution/duration.
- [ ] **Content Adaptation**: Auto-scale or warn when horizontal content doesn't fit portrait orientation.
- [ ] **Safe Zones**: Respect platform UI overlays (captions, profile pics) with configurable safe zones.
- [ ] **Swipe Animations**: Optional slide transition effects optimized for vertical scrolling behavior.

## Authoring & UX for Export

- [ ] **Export Subcommands**: Add `lantern export-image` and `lantern export-video` commands with shared flags (theme, range).
- [ ] **Frontmatter Controls**: Support per-deck/per-slide frontmatter: `fps`, `default_duration`, `transition`, `record: true`.
- [ ] **Deterministic Seeds**: Add `--seed` for any animations (typing jitter, cursor blink timing) to keep exports repeatable.
- [ ] **Preset Profiles**: Presets like `social-card`, `doc-screenshot`, `talk-demo` mapping to resolution + theme.
