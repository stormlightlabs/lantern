# Introduction

Lantern is a terminal presentation tool for markdown decks. It focuses on fast rendering, readable
defaults, and plain-text decks that still look good in a terminal.

The current implementation supports interactive presenting, formatted printing, theme selection,
syntax-highlighted code, tables, images, and admonitions. The code is split into two crates:

- `lantern-core` parses markdown, metadata, themes, highlighting, printing, and validation.
- `lantern-cli` exposes the `lantern` command and renders the interactive terminal UI with ratatui.

Lantern is still early.

Some planned features in the roadmap, such as live reload, search UI, speaker note parsing,
and export commands, are not implemented yet.
