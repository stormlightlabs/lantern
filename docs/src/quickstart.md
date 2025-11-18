# Quickstart

Get started with lantern in minutes.

## Installation

Currently, you'll need to build from source:

```bash
git clone https://github.com/yourusername/lantern.git
cd lantern
cargo build --release
```

The binary will be available at `target/release/lantern`.

## Creating Your First Deck

Create a new markdown file called `presentation.md`:

````markdown
---
theme: default
author: Your Name
---

# Welcome to Slides

A modern terminal-based presentation tool

---

## Features

- Parse markdown into slides
- Interactive TUI navigation with full keyboard support
- Speaker notes with toggle visibility
- Live presentation timer
- Status bar with slide count and navigation hints
- Print to stdout
- Syntax highlighting (coming soon)

---

## Code Example

```rust
fn main() {
    println!("Hello, lantern!");
}
```

Supports multiple languages with syntax highlighting.

---

## Lists and Formatting

- Unordered lists with bullets
- **Bold text** for emphasis
- *Italic text* for style
- `inline code` for commands

---

# Thank You

Questions?
````

## Presenting Your Slides

Run the interactive TUI presenter:

```bash
lantern present presentation.md
```

### Navigation Keys

- `→`, `j`, `Space`, `n` - Next slide
- `←`, `k`, `p` - Previous slide
- `Shift+N` - Toggle speaker notes
- `q`, `Ctrl+C`, `Esc` - Quit presentation

## Printing to Stdout

Print all slides to stdout with formatting:

```bash
lantern print presentation.md
```

Adjust output width:

```bash
lantern print presentation.md --width 100
```

Use a specific theme:

```bash
lantern print presentation.md --theme nord
```

## Slide Separators

Slides are separated by three dashes on a line by themselves:

```markdown
# Slide 1

Content here

---

# Slide 2

More content
```

## Front Matter

Optional metadata at the start of your file:

YAML format:

```yaml
---
theme: dark
author: Jane Doe
---
```

TOML format:

```toml
+++
theme = "monokai"
author = "John Smith"
+++
```

## Supported Markdown

Currently supported:

- Headings (H1-H6)
- Paragraphs with inline formatting (bold, italic, strikethrough, code)
- Code blocks with language tags
- Lists (ordered and unordered with nesting)
- Horizontal rules
- Blockquotes
- Tables with automatic column width calculation and proper Unicode borders

## Speaker Notes

Add speaker notes to any slide using the `::: notes` directive:

```markdown
# Your Slide Title

Main content visible to the audience.

::: notes
These are your speaker notes.
Press Shift+N to toggle their visibility.
They appear in a separate panel during presentation.
:::
```

## Status Bar

The status bar at the bottom displays:

- Filename of the current presentation
- Current slide number / Total slides
- Active theme name
- Navigation hints
- Notes visibility indicator (✓ when shown)
- Elapsed presentation time (HH:MM:SS)

## Environment Variables

Customize defaults with environment variables:

```bash
# Set default theme
export LANTERN_THEME=nord

# Set default author (used if not in frontmatter)
export USER=YourName
```

## Themes

See the [Themes](./appendices/themes.md) reference for details on all available themes and customization options.
