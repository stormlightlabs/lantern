# Themes

Lantern uses embedded [Base16](https://github.com/chriskempson/base16) themes.

Each theme defines 16 colors, then lantern maps those colors to slide content,
syntax highlighting, admonitions, and UI chrome.

## Available themes

Built-in theme names:

- `catppuccin-latte`
- `catppuccin-mocha`
- `gruvbox-material-dark`
- `gruvbox-material-light`
- `nord`
- `nord-light`
- `oxocarbon-dark`
- `oxocarbon-light`
- `solarized-dark`
- `solarized-light`

`default` is also accepted. It resolves to `oxocarbon-dark` or `oxocarbon-light` based on terminal
background detection.

Named themes are loaded from the user config directory first, then from the built-in list.
Unknown theme names fail with an error when presenting or printing.

## Using a theme

### Front matter

```markdown
---
theme: catppuccin-mocha
---

# First slide
```

TOML front matter works too:

```markdown
+++
theme = "nord"
+++

# First slide
```

### Command line

```bash
lantern present presentation.md --theme nord
lantern print presentation.md --theme catppuccin-latte
lantern present presentation.md --theme-file ./my-theme.yml
```

### User config directory

Put Base16 YAML themes in one of these directories and reference them by file stem with `--theme`
or front matter:

- `$LANTERN_CONFIG_HOME/themes`
- `$XDG_CONFIG_HOME/lantern/themes`
- `~/.config/lantern/themes`

For example, `~/.config/lantern/themes/acme.yml` can be used as `--theme acme` or
`theme: acme`.

### Environment variable

```bash
export SLIDES_THEME=gruvbox-material-dark
lantern present presentation.md
```

## Theme priority

Lantern resolves the theme in this order:

1. `--theme-file` on the command line
2. `--theme` on the command line
3. `theme` in front matter
4. `SLIDES_THEME`
5. Detected default, `oxocarbon-dark` or `oxocarbon-light`

## Base16 mapping

Content colors:

- Headings: `base0D`
- Body text: `base05`
- Bold text: `base0E`
- Italic text: `base09`
- Code blocks: `base0B`
- Inline code background: `base02`
- Links: `base0C`
- List markers: `base0A`
- Rules, borders, and dimmed text: `base03`

UI colors:

- UI background: `base00`
- Borders and status bar background: `base02`
- UI titles: `base06`
- UI text: `base05`

Admonition colors:

- Note: `base0D`
- Tip: `base0E`
- Warning: `base0A`
- Danger: `base08`
- Success: `base0B`
- Info: `base0C`

## Validate a theme file

Lantern can validate a Base16 YAML file:

```bash
lantern check --theme theme.yml
```

Custom theme files can be validated, but rendering still uses built-in themes.
The same file shape can be rendered with `--theme-file` or from the user config theme directory.

A valid theme uses this shape:

```yaml
system: "base16"
name: "My Theme"
author: "Your Name"
variant: "dark"
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

## Rendering notes

The printer and TUI use Unicode symbols for readable terminal output:

- `▉ ▓ ▒ ░ ▌` for heading levels
- `─` and `═` for horizontal rules
- `│` for blockquotes and table dividers
- `•` for unordered lists

Code blocks use Syntect for syntax highlighting.
Lantern chooses a light or dark Syntect theme based on the active lantern theme variant.
