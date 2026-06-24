# Quickstart

## Installation

From a local clone:

```bash
git clone https://github.com/stormlightlabs/lantern.git
cd lantern
cargo install --path crates/cli
```

You can also run it without installing:

```bash
cargo run -p lantern-cli -- present presentation.md
```

The installed binary is named `lantern`.

## Create a deck

Create `presentation.md`:

````markdown
---
theme: nord
author: Your Name
---

# Welcome to lantern

A terminal presentation tool built with Rust.

---

## Features

- Markdown slides split with `---`
- Base16 themes
- Syntax-highlighted code blocks
- Tables, lists, blockquotes, and horizontal rules
- Image rendering in terminals supported by `ratatui-image`
- GitHub and Obsidian-style admonitions

---

## Code

```rust
fn main() {
    println!("Hello, lantern!");
}
```

---

## Images

![A local image](./image.png)

---

## Done

Press `q` to quit.
````

## Present

```bash
lantern present presentation.md
```

Use a theme from the command line:

```bash
lantern present presentation.md --theme catppuccin-mocha
```

## Print

Print a deck to stdout:

```bash
lantern print presentation.md
```

Set the output width:

```bash
lantern print presentation.md --width 100
```

## Check a deck

Validate a slide deck:

```bash
lantern check presentation.md
```

Run stricter checks:

```bash
lantern check presentation.md --strict
```

Validate a Base16 theme file:

```bash
lantern check --theme theme.yml
```

## Navigation

| Key                    | Action                                       |
| ---------------------- | -------------------------------------------- |
| `→`, `j`, `Space`, `n` | Next slide                                   |
| `←`, `k`, `p`          | Previous slide                               |
| `?`                    | Toggle help line                             |
| `Shift+N`              | Toggle notes panel if a slide contains notes |
| `q`, `Ctrl+C`, `Esc`   | Quit                                         |

The input layer reserves `/` and `Ctrl+F` for search, but the search UI is not implemented yet.

## Slide separators

Put three dashes on a line by themselves between slides:

```markdown
# Slide 1

Content here.

---

# Slide 2

More content.
```

Separators inside fenced code blocks are ignored.

## Front matter

Lantern accepts YAML front matter:

```yaml
---
theme: oxocarbon-dark
author: Jane Doe
date: 2026-06-24
paging: "Slide %d / %d"
---
```

It also accepts TOML front matter:

```toml
+++
theme = "nord"
author = "Jane Doe"
+++
```

Current metadata fields are `theme`, `author`, `date`, and `paging`.

Theme selection is implemented.

The UI currently uses a fixed status-bar format, so `author`, `date`, and `paging` are parsed but
not displayed in the presenter.

## Supported markdown

- Headings `#` through `######`
- Paragraphs
- Bold, italic, strikethrough, and inline code
- Fenced and indented code blocks
- Ordered and unordered lists with nesting
- Horizontal rules
- Blockquotes
- Tables
- Images
- GitHub and Obsidian-style admonitions

Speaker notes are represented in the internal slide model and can be toggled in the UI, but
markdown parsing for `::: notes` has not been implemented yet.

## Environment variables

Set the default theme with `SLIDES_THEME`:

```bash
export SLIDES_THEME=nord
```

If no author appears in front matter, lantern uses `USER` or `USERNAME`.

## More reference

- [Themes](./appendices/themes.md)
- [Extensions](./extensions.md)
- [Logging](./logging.md)
