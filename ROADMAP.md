# Slides

## Plumbing

__Objective:__ Establish a clean, testable core with `clap` and a minimal `ratatui` loop.

| Task                         | Description                                                                                                                                          | Key Crates                                                               |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| __✓ Project Scaffolding__    | Initialize workspace with `slides-core`, `slides-cli`, and `slides-ui` crates. Use `cargo-generate` and a `justfile` for scripts.                    | `cargo`, `just`, `clap`                                                  |
| __✓ CLI Definition__         | Implement root command `slides` with subcommands:<br>• `present` (TUI)<br>• `print` (stdout)<br>• `init` (scaffold deck)<br>• `check` (lint slides). | [`clap`](https://docs.rs/clap/latest/clap/)                              |
| __✓ Logging & Colors__       | Integrate structured logs via `tracing`.<br>Use __owo-colors__ for color abstraction (no dynamic dispatch).                                          | [`owo-colors`](https://docs.rs/owo-colors/latest/owo_colors/), `tracing` |
| __✓ Terminal & Event Setup__ | Configure alternate screen, raw mode, input loop, resize handler.                                                                                    | [`crossterm`](https://docs.rs/crossterm/latest/crossterm/), `ratatui`    |
| __CI/CD + Tooling__          | Setup `cargo fmt`, `clippy`, `test`, and `cross` matrix CI.                                                                                          | GitHub Actions                                                           |

## Data Model (Parser & Slides)

__Objective:__ Parse markdown documents into a rich `Slide` struct.

| Task                   | Description                                                                                                    | Key Crates                                                                |
| ---------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| __✓ Parser Core__      | Split files on `---` separators.<br>Detect title blocks, lists, and code fences.<br>Represent as `Vec<Slide>`. | [`pulldown-cmark`](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/) |
| __✓ Slide Model__      | Define structs: `Slide`, `Block`, `TextSpan`, `CodeBlock`, etc.                                                | Internal                                                                  |
| __✓ Metadata Parsing__ | Optional front matter (YAML/TOML) for theme, author, etc.                                                      | [`serde_yml`](https://docs.rs/serde_yml)                                  |
| __Error & Validation__ | Provide friendly parser errors with file/line info.                                                            | [`thiserror`](https://docs.rs/thiserror)                                  |
| __Basic CLI UX__       | `slides present file.md` runs full TUI.<br>`slides print` renders to stdout with width constraint.             | `clap`                                                                    |

---

## Rendering & Navigation

__Objective:__ Build the interactive slide renderer with navigation.

| Task                    | Description                                                                                         | Key Crates                                           |
| ----------------------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| __Ratatui Integration__ | Build basic slide viewer using layout, blocks, paragraphs.                                          | [`ratatui`](https://docs.rs/ratatui/latest/ratatui/) |
| __Input & State__       | Support `←/→`, `j/k`, `q`, numeric jumps, and window resize.                                        | `crossterm`, `ratatui`                               |
| __Status Bar__          | Display slide count, filename, clock, and theme name.                                               | `ratatui`                                            |
| __Color Styling__       | Apply consistent color palette via `owo-colors`. Define traits like `ThemeColor` for strong typing. | `owo-colors`                                         |
| __Configurable Themes__ | Support themes via TOML files mapping semantic roles (`heading`, `body`, `accent`) → color pairs.   | `toml`, `serde`                                      |

## Code Highlighting via Syntect

__Objective:__ Add first-class syntax highlighting using Syntect.

| Task            | Description                                                                                                              | Key Crates                                           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------- |
| __Syntect__     | Load `.tmTheme` / `.sublime-syntax` definitions on startup.<br>Cache `SyntaxSet` + `ThemeSet`.                           | [`syntect`](https://docs.rs/syntect/latest/syntect/) |
| __Code Blocks__ | Detect fenced code blocks with language tags.<br>Render syntax-highlighted text with color spans mapped to `owo-colors`. | `syntect`, `owo-colors`                              |
| __Theming__     | Map terminal theme choice to Syntect theme (e.g., `"OneDark"`, `"SolarizedDark"`, `"Monokai"`).                          | `syntect`                                            |
| __Performance__ | Lazy-load themes and syntaxes; use `once_cell` for caching.                                                              | `once_cell`                                          |
| __Mode__        | Render to ANSI-colored plain text output (for `slides print`).                                                           | `owo-colors`                                         |

## Presenter

__Objective:__ Introduce features for live presentations and authoring convenience.

| Task                 | Description                                                   | Key Crates                                        |
| -------------------- | ------------------------------------------------------------- | ------------------------------------------------- |
| __Speaker Notes__    | `n` toggles speaker notes (parsed via `::: notes`).           | `ratatui`                                         |
| __Timer & Progress__ | Session timer + per-slide progress bar.                       | `ratatui`, `chrono`                               |
| __Live Reload__      | File watcher auto-refreshes content.                          | [`notify`](https://docs.rs/notify/latest/notify/) |
| __Search__.          | Fuzzy find slide titles via `ctrl+f`.                         | [`fuzzy-matcher`](https://docs.rs/fuzzy-matcher)  |
| __Theme Commands__   | CLI flag `--theme <name>` switches both Syntect + owo themes. | `clap`, internal `ThemeRegistry`                  |

## Markdown Extension

__Objective:__ Add richness and visual polish to text and layout.

| Task                 | Description                                                  | Key Crates                    |
| -------------------- | ------------------------------------------------------------ | ----------------------------- |
| __Tables & Lists__   | Render GitHub-style tables, bullets, and task lists.         | `pulldown-cmark`, `ratatui`   |
| __Admonitions__      | Highlighted boxes with icons                                 | `owo-colors`, internal glyphs |
| __Horizontal Rules__ | Use box-drawing (`─`, `═`) and shading (`░`, `▓`).           | Unicode constants             |
| __Generators__.      | `slides init` scaffolds an example deck with code and notes. | `include_str!`, `fs`          |

## RC

| Task                 | Description                                                        | Key Crates                   |
| -------------------- | ------------------------------------------------------------------ | ---------------------------- |
| __Config Discovery__ | Read from `$XDG_CONFIG_HOME/slides/config.toml` for defaults.      | `dirs`, `serde`              |
| __Theme Registry__   | Built-in theme manifest (e.g., `onedark`, `solarized`, `plain`).   | Internal                     |
| __Release__          | Tag `v1.0.0-rc.1` with changelog and binaries for major platforms. | `cargo-dist`, GitHub Actions |
