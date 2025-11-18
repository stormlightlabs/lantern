# Themes

slides.rs provides a theme system for customizing the appearance of your presentations. Themes control colors and styling for headings, body text, code blocks, and UI elements.

## Automatic Light/Dark Detection

Each theme automatically detects your terminal background and selects the appropriate light or dark variant. This ensures optimal contrast and readability regardless of your terminal settings.

## Available Theme Schemes

The following color schemes are built-in:

**basic** (default) - IBM's Oxocarbon color palette with clean, modern styling

- Dark variant: Light text on dark background with vibrant accents
- Light variant: Dark text on light background with adjusted colors

**monokai** - Inspired by the popular Monokai editor theme

- Dark variant: Classic Monokai with pink headings and green code
- Light variant: Adjusted colors for light backgrounds

**dracula** - Based on the Dracula color scheme

- Dark variant: Purple and cyan tones optimized for dark backgrounds
- Light variant: Darker variants of Dracula colors for light backgrounds

**solarized** - Ethan Schoonover's Solarized palette

- Dark variant: Solarized Dark with blue headings
- Light variant: Solarized Light with adjusted foreground colors

**nord** - Arctic-inspired theme with cool tones

- Dark variant: Subtle blues and greens on dark background
- Light variant: Nord colors adjusted for light backgrounds

## Using Themes

### Via Frontmatter

Specify a theme in your slide deck's YAML frontmatter:

````markdown
---
theme: monokai
---

# Your First Slide

Content here
```

The terminal background will be automatically detected. To force a specific variant:

```markdown
---
theme: solarized:light
---
```

Or with TOML frontmatter:

```markdown
+++
theme = "dracula:dark"
+++

# Your First Slide

Content here
````

### Via Command Line

Override the theme with the `--theme` flag:

```bash
# Auto-detect terminal background
slides present slides.md --theme nord
slides print slides.md --theme solarized

# Force a specific variant
slides present slides.md --theme monokai:light
slides print slides.md --theme nord:dark
```

### Via Environment Variable

Set a default theme using the `SLIDES_THEME` environment variable:

```bash
# Auto-detect variant
export SLIDES_THEME=basic
slides present slides.md

# Force specific variant
export SLIDES_THEME=dracula:dark
slides present slides.md
```

## Theme Priority

When multiple theme sources are specified, the priority order is:

1. Command line flag (`--theme`)
2. Frontmatter metadata (`theme:` field)
3. Environment variable (`SLIDES_THEME`)
4. Default theme

## Theme Components

Each theme defines colors for:

- Headings (level 1-6)
- Body text
- Accent colors
- Code blocks and inline code
- Code fence markers
- Horizontal rules
- List markers (bullets and numbers)
- Blockquote borders
- Table borders

## Rendering Features

The printer uses Unicode box-drawing characters for clean visual output:

- `─` and `═` for horizontal lines
- `│` for vertical borders
- `┼` for table intersections
- `•` for unordered list markers

Tables automatically calculate column widths based on content and available terminal width.

## Default Theme

The application's default slide theme is based on Oxocarbon

### Dark

```yml
- scheme: "Oxocarbon Dark"
  author: "shaunsingh/IBM"
  palette:
    base00: "#161616"
    base01: "#262626"
    base02: "#393939"
    base03: "#525252"
    base04: "#dde1e6"
    base05: "#f2f4f8"
    base06: "#ffffff"
    base07: "#08bdba"
    base08: "#3ddbd9"
    base09: "#78a9ff"
    base0A: "#ee5396"
    base0B: "#33b1ff"
    base0C: "#ff7eb6"
    base0D: "#42be65"
    base0E: "#be95ff"
    base0F: "#82cfff"
```

### Light

```yml
- scheme: "Oxocarbon Light"
  author: "shaunsingh/IBM"
  palette:
    base00: "#f2f4f8"
    base01: "#dde1e6"
    base02: "#525252"
    base03: "#161616"
    base04: "#262626"
    base05: "#393939"
    base06: "#525252"
    base07: "#08bdba"
    base08: "#ff7eb6"
    base09: "#ee5396"
    base0A: "#FF6F00"
    base0B: "#0f62fe"
    base0C: "#673AB7"
    base0D: "#42be65"
    base0E: "#be95ff"
    base0F: "#37474F"
```
