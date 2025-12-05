# lantern

> A modern, fast, terminal presentation tool inspired by [`maaslalani/slides`](https://github.com/maaslalani/slides), built with Rust.

<details>
<summary>
Now with image support (if your terminal supports it!)
</summary>

![Rendered on Ghostty](./assets/ghostty.png)

![Rendered on iTerm2](./assets/iterm2.png)

</details>

## Quickstart

### Installation

From a local clone:

```bash
cargo install --path cli
```

From GitHub:

```bash
cargo install --git https://github.com/stormlightlabs/lantern.git lantern-cli
```

From Tangled:

```bash
cargo install --git https://tangled.sh/desertthunder.dev/lantern lantern-cli
```

### Create Your First Deck

Create a markdown file `presentation.md`:

````markdown
---
theme: nord
---

# Welcome to lantern

A terminal presentation tool built with Rust

---

## Features

- Base16 theming system
- Syntax highlighting
- Live reload
- Export to image/video

---

## Code Example

```rust
fn main() {
    println!("Hello, lantern!");
}
```

---

## That's it

Press `q` to quit, `←/→` to navigate

````

### Present

```bash
# Interactive TUI mode
lantern present presentation.md

# Print to stdout
lantern print presentation.md

# With custom theme
lantern present presentation.md --theme catppuccin-mocha
```

### Navigation

| Key           | Action              |
| ------------- | ------------------- |
| `→`, `j`, `n` | Next slide          |
| `←`, `k`, `p` | Previous slide      |
| `q`           | Quit                |

## Design Principles

__Color as Data:__
All color use flows through typed wrappers using `owo-colors`. No ad-hoc ANSI escapes.

__Themeable:__
Built on the [Base16](https://github.com/chriskempson/base16) theming system with 10 prebuilt themes (Catppuccin, Nord, Gruvbox Material, Solarized, Oxocarbon). Each theme defines 16 semantic colors mapped to content and UI elements. Themes can be selected via frontmatter, CLI flags, or environment variables.

__Reproducible:__
Everything is reproducible in plain text — decks can render without TUI (using `lantern print`).

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
