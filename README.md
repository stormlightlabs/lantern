# slides.rs

> A modern, fast, terminal presentation tool inspired by [`maaslalani/slides`](https://github.com/maaslalani/slides), built with Rust.

## Design Principles

__Color as Data:__
All color use flows through typed wrappers using `owo-colors`. No ad-hoc ANSI escapes.

__Themeable:__
Multiple built-in color schemes (basic, monokai, dracula, solarized, nord) with automatic light/dark variant detection based on terminal background. Themes can be selected via frontmatter, CLI flags, or environment variables, with optional explicit variant override using `:light` or `:dark` suffix.

__Reproducible:__
Everything is reproducible in plain text — decks can render without TUI (using `slides print`).

__Composable:__
Parser → Model → Renderer are independent modules with tests and traits.

__Portable:__
Runs on any terminal supporting UTF-8; dependencies limited to core crates.

## Testing

This project uses `cargo-llvm-cov` for coverage

Installation:

```sh
# MacOS
brew install cargo-llvm-cov

# Linux
cargo +stable install cargo-llvm-cov --locked
```

Run tests:

```sh
cargo llvm-cov

# Open the browser
cargo llvm-cov --open
```
