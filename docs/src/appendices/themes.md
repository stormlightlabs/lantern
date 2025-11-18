# Themes

slides.rs uses the [Base16](https://github.com/chriskempson/base16) theming system for customizing the appearance of your presentations. Base16 provides a standardized way to define color schemes that work consistently across dark and light backgrounds.

## Base16 Color System

Base16 defines 16 semantic colors (base00 through base0F) that serve specific purposes:

### Background Shades

- **base00-03**: Background colors from darkest to lighter (or lightest to darker in light themes)

### Foreground Shades

- **base04-07**: Foreground colors from darker to lightest (or lightest to darker in light themes)

### Accent Colors

- **base08**: Red (variables, deletion)
- **base09**: Orange (integers, constants, emphasis)
- **base0A**: Yellow (classes, list markers)
- **base0B**: Green (strings, code blocks)
- **base0C**: Cyan (links, support functions)
- **base0D**: Blue (functions, headings)
- **base0E**: Magenta (keywords, strong emphasis)
- **base0F**: Brown (deprecated, special)

## Color Mapping

slides.rs maps base16 colors to semantic roles:

### Content Colors

- **Headings** (base0D): Blue accent for slide titles
- **Body text** (base05): Main foreground color
- **Strong/Bold** (base0E): Magenta for emphasis
- **Emphasis/Italic** (base09): Orange for subtle emphasis
- **Code blocks** (base0B): Green for fenced code
- **Inline code background** (base02): Selection background
- **Links** (base0C): Cyan for hyperlinks
- **Accents** (base08): Red for highlights
- **List markers** (base0A): Yellow for bullets/numbers
- **Dimmed elements** (base03): Comments, borders, rules

### UI Chrome Colors

- **UI background** (base00): Status bar and UI backgrounds
- **UI borders** (base04): Window and panel borders
- **UI titles** (base06): Bright text for UI elements
- **UI text** (base07): Brightest text for status bars

## Available Themes

slides.rs includes 10 prebuilt base16 themes, embedded at compile time:

### Catppuccin

- **catppuccin-mocha** - Dark theme with pastel colors
- **catppuccin-latte** - Light theme with warm tones

### Gruvbox Material

- **gruvbox-material-dark** - Retro dark theme with warm colors
- **gruvbox-material-light** - Retro light theme

### Nord

- **nord** - Arctic-inspired dark theme with cool blues
- **nord-light** - Nord palette adapted for light backgrounds

### Oxocarbon

- **oxocarbon-dark** - IBM's modern dark theme (default)
- **oxocarbon-light** - IBM's modern light theme

### Solarized

- **solarized-dark** - Ethan Schoonover's precision dark palette
- **solarized-light** - Solarized adapted for light backgrounds

## Using Themes

### Via Frontmatter

Specify a theme in your slide deck's YAML frontmatter:

```markdown
---
theme: catppuccin-mocha
---

# Your First Slide

Content here
```

Or with TOML frontmatter:

```markdown
+++
theme = "nord"
+++

# Your First Slide
```

### Via Command Line

Override the theme with the `--theme` flag:

```bash
slides present slides.md --theme nord
slides print slides.md --theme catppuccin-latte
```

### Via Environment Variable

Set a default theme using the `SLIDES_THEME` environment variable:

```bash
export SLIDES_THEME=gruvbox-material-dark
slides present slides.md
```

## Theme Priority

When multiple theme sources are specified, the priority order is:

1. Command line flag (`--theme`)
2. Frontmatter metadata (`theme:` field)
3. Environment variable (`SLIDES_THEME`)
4. Default theme (nord for dark terminals, nord-light for light terminals)

## Custom Themes (Coming Soon)

Future versions will support loading custom base16 YAML themes from:

- `~/.config/slides/themes/` directory
- `--theme-file` command line flag

Base16 YAML format:

```yaml
system: "base16"
name: "My Custom Theme"
author: "Your Name"
variant: "dark"  # or "light"
palette:
  base00: "#1a1b26"
  base01: "#16161e"
  base02: "#2f3549"
  base03: "#444b6a"
  base04: "#787c99"
  base05: "#a9b1d6"
  base06: "#cbccd1"
  base07: "#d5d6db"
  base08: "#c0caf5"
  base09: "#a9b1d6"
  base0A: "#0db9d7"
  base0B: "#9ece6a"
  base0C: "#b4f9f8"
  base0D: "#2ac3de"
  base0E: "#bb9af7"
  base0F: "#f7768e"
```

You can find thousands of base16 themes at the [Base16 Gallery](https://tinted-theming.github.io/tinted-gallery/).

## Rendering Features

The printer uses Unicode box-drawing characters for clean visual output:

- `▉ ▓ ▒ ░ ▌` for heading levels (h1-h6)
- `─` and `═` for horizontal rules
- `│` for blockquote borders and table dividers
- `•` for unordered list markers

Tables automatically calculate column widths based on content and available terminal width.

Code blocks support syntax highlighting through [Syntect](https://github.com/trishume/syntect), which automatically adapts to your selected theme's light/dark variant.
