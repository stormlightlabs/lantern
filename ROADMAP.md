# Slides

## Plumbing

__Objective:__ Establish a clean, testable core with `clap` and a minimal `ratatui` loop.

| Task                         | Description                                                                    | Key Crates                  |
| ---------------------------- | ------------------------------------------------------------------------------ | --------------------------- |
| __✓ Project Scaffolding__    | Initialize workspace with `slides-core`, `slides-cli`, and `slides-ui` crates. | `cargo`, `just`, `clap`     |
|                              | Use `cargo-generate` and a `justfile` for scripts.                             |                             |
| __✓ CLI Definition__         | Implement root command `slides` with subcommands:                              | `clap`[^1]                  |
|                              | • `present` (TUI)                                                              |                             |
|                              | • `print` (stdout)                                                             |                             |
|                              | • `init` (scaffold deck)                                                       |                             |
|                              | • `check` (lint slides).                                                       |                             |
| __✓ Logging & Colors__       | Integrate structured logs via `tracing`.                                       | `owo-colors`[^2], `tracing` |
|                              | Use __owo-colors__ for color abstraction (no dynamic dispatch).                |                             |
| __✓ Terminal & Event Setup__ | Configure alternate screen, raw mode, input loop, resize handler.              | `crossterm`[^3], `ratatui`  |
| __CI/CD + Tooling__          | Setup `cargo fmt`, `clippy`, `test`, and `cross` matrix CI.                    | GitHub Actions              |

## Data Model (Parser & Slides)

__Objective:__ Parse markdown documents into a rich `Slide` struct.

| Task                   | Description                                                     | Key Crates           |
| ---------------------- | --------------------------------------------------------------- | -------------------- |
| __✓ Parser Core__      | Split files on `---` separators.                                | `pulldown-cmark`[^4] |
|                        | Detect title blocks, lists, and code fences.                    |                      |
|                        | Represent as `Vec<Slide>`.                                      |                      |
| __✓ Slide Model__      | Define structs: `Slide`, `Block`, `TextSpan`, `CodeBlock`, etc. | Internal             |
| __✓ Metadata Parsing__ | Optional front matter (YAML/TOML) for theme, author, etc.       | `serde_yml`[^5]      |
| __Error & Validation__ | Provide friendly parser errors with file/line info.             | `thiserror`[^6]      |
| __✓ Basic CLI UX__     | `slides present file.md` runs full TUI.                         | `clap`               |
|                        | `slides print` renders to stdout with width constraint.         |                      |

## Rendering & Navigation

__Objective:__ Build the interactive slide renderer with navigation.

| Task                      | Description                                                                                       | Key Crates             |
| ------------------------- | ------------------------------------------------------------------------------------------------- | ---------------------- |
| __✓ Ratatui Integration__ | Build basic slide viewer using layout, blocks, paragraphs.                                        | `ratatui`[^7]          |
| __✓ Input & State__       | Support `←/→`, `j/k`, `q`, numeric jumps, and window resize.                                      | `crossterm`, `ratatui` |
| __✓ Status Bar__          | Display slide count, filename, clock, and theme name.                                             | `ratatui`              |
| __✓ Color Styling__       | Apply consistent color palette via `owo-colors`. Define traits like `ThemeColor`.                 | `owo-colors`           |
| __Configurable Themes__   | Support themes via TOML files mapping semantic roles (`heading`, `body`, `accent`) → color pairs. | `toml`, `serde`        |

---

## Code Highlighting via Syntect

__Objective:__ Add first-class syntax highlighting using Syntect.

| Task            | Description                                                                  | Key Crates              |
| --------------- | ---------------------------------------------------------------------------- | ----------------------- |
| __Syntect__     | Load `.tmTheme` / `.sublime-syntax` definitions on startup.                  | `syntect`[^8]           |
|                 | Cache `SyntaxSet` + `ThemeSet`.                                              |                         |
| __Code Blocks__ | Detect fenced code blocks with language tags.                                | `syntect`, `owo-colors` |
|                 | Render syntax-highlighted text with color spans mapped to `owo-colors`.      |                         |
| __Theming__     | Map terminal theme choice to Syntect theme (e.g., `"OneDark"`, `"Monokai"`). | `syntect`               |
| __Performance__ | Lazy-load themes and syntaxes; use `once_cell` for caching.                  | `once_cell`             |
| __Mode__        | Render to ANSI-colored plain text output (for `slides print`).               | `owo-colors`            |

## Presenter

__Objective:__ Introduce features for live presentations and authoring convenience.

| Task                 | Description                                                   | Key Crates                       |
| -------------------- | ------------------------------------------------------------- | -------------------------------- |
| __Speaker Notes__    | `n` toggles speaker notes (parsed via `::: notes`).           | `ratatui`                        |
| __Timer & Progress__ | Session timer + per-slide progress bar.                       | `ratatui`, `chrono`              |
| __Live Reload__      | File watcher auto-refreshes content.                          | `notify`[^9]                     |
| __Search__           | Fuzzy find slide titles via `ctrl+f`.                         | `fuzzy-matcher`[^10]             |
| __Theme Commands__   | CLI flag `--theme <name>` switches both Syntect + owo themes. | `clap`, internal `ThemeRegistry` |

## Markdown Extension

__Objective:__ Add richness and visual polish to text and layout.

| Task                 | Description                                                  | Key Crates                    |
| -------------------- | ------------------------------------------------------------ | ----------------------------- |
| __Tables & Lists__   | Render GitHub-style tables, bullets, and task lists.         | `pulldown-cmark`, `ratatui`   |
| __Admonitions__      | Highlighted boxes with icons                                 | `owo-colors`, internal glyphs |
| __Horizontal Rules__ | Use box-drawing (`─`, `═`) and shading (`░`, `▓`).           | Unicode constants             |
| __Generators__       | `slides init` scaffolds an example deck with code and notes. | `include_str!`, `fs`          |

## RC

| Task                 | Description                                                        | Key Crates                   |
| -------------------- | ------------------------------------------------------------------ | ---------------------------- |
| __Config Discovery__ | Read from `$XDG_CONFIG_HOME/slides/config.toml` for defaults.      | `dirs`, `serde`              |
| __Theme Registry__   | Built-in theme manifest (e.g., `onedark`, `solarized`, `plain`).   | Internal                     |
| __Release__          | Tag `v1.0.0-rc.1` with changelog and binaries for major platforms. | `cargo-dist`, GitHub Actions |

[^1]: <https://docs.rs/clap/latest/clap/>
[^2]: <https://docs.rs/owo-colors/latest/owo_colors/>
[^3]: <https://docs.rs/crossterm/latest/crossterm/>
[^4]: <https://docs.rs/pulldown-cmark/latest/pulldown_cmark/>
[^5]: <https://docs.rs/serde_yml>
[^6]: <https://docs.rs/thiserror>
[^7]: <https://docs.rs/ratatui/latest/ratatui/>
[^8]: <https://docs.rs/syntect/latest/syntect/>
[^9]: <https://docs.rs/notify/latest/notify/>
[^10]: <https://docs.rs/fuzzy-matcher>
