# Introduction

Lantern is a terminal presentation tool for markdown decks. It focuses on fast rendering, readable
defaults, and plain-text decks that still look good in a terminal.

The current implementation supports interactive presenting, formatted printing, theme selection,
syntax-highlighted code, tables, images, and admonitions. The published package contains a library
and a binary:

- The `lantern_cli` library parses markdown, metadata, themes, highlighting, printing, validation,
  and terminal UI rendering.
- The `lantern` binary exposes the command-line interface.

Lantern is still early.

Some planned features in the roadmap, such as live reload, search UI, speaker note parsing,
and export commands, are not implemented yet.
