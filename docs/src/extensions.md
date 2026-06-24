# Extensions

Lantern currently has one markdown extension: admonitions. These are highlighted blocks for notes,
warnings, tips, and similar callouts.

## Supported types

| Type        | Aliases                |
| ----------- | ---------------------- |
| `note`      |                        |
| `info`      |                        |
| `tip`       | `hint`                 |
| `important` |                        |
| `warning`   | `caution`, `attention` |
| `danger`    | `error`                |
| `success`   | `check`, `done`        |
| `question`  | `help`, `faq`          |
| `example`   |                        |
| `quote`     |                        |
| `abstract`  | `summary`, `tldr`      |
| `todo`      |                        |
| `bug`       |                        |
| `failure`   | `fail`, `missing`      |

## GitHub and Obsidian syntax

```markdown
> [!NOTE]
> Useful context for the slide.

> [!TIP]
> A practical suggestion.

> [!IMPORTANT]
> Something the audience should not miss.

> [!WARNING]
> A risk or caveat.

> [!CAUTION]
> A stronger warning.
```

You can add a title after the marker:

```markdown
> [!WARNING] Production caveat
> This path assumes the database is already migrated.
```

Obsidian-style lowercase markers work too:

```markdown
> [!quote]
> The quote text.

> [!example]
> Example content.

> [!bug]
> Known issue.
```

## Fence syntax

Lantern also accepts a simple fence form:

```markdown
:::note
This is a note.
:::

:::warning
This is a warning.
:::

:::tip
This is a tip.
:::
```

Fence admonitions currently accept the type only.

Custom titles are supported for the blockquote form, not for fence admonitions.

## Rendering

Admonitions render with themed colors, Unicode icons, and rounded borders.

Their content is parsed into lantern's normal slide blocks, so paragraphs, lists,
and code blocks can appear inside an admonition.
