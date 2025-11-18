# Extensions

## Admonitions/Alerts

Admonitions (also called alerts or callouts) are special highlighted blocks that draw attention to important information. Lantern supports both GitHub-flavored markdown syntax and a custom fence syntax.

### Supported Types

All admonitions are rendered with colored borders and icons:

- **Note/Info** - Blue - General information
- **Tip/Hint/Important** - Purple - Helpful suggestions
- **Warning/Caution/Attention** - Yellow - Important warnings
- **Danger/Error** - Red - Critical issues
- **Success/Check/Done** - Green - Success messages
- **Question/Help/FAQ** - Cyan - Questions and help
- **Example** - Green - Example content
- **Quote** - Cyan - Quotations
- **Abstract/Summary/TLDR** - Blue - Summaries
- **Todo** - Cyan - Todo items
- **Bug** - Red - Bug reports
- **Failure/Fail/Missing** - Red - Failures

### GitHub/Obsidian Syntax

```markdown
> [!NOTE]
> Useful information that users should know, even when skimming content.

> [!TIP]
> Helpful advice for doing things better or more easily.

> [!IMPORTANT]
> Key information users need to know to achieve their goal.

> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.

> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.
```

### Obsidian

```markdown
> [!quote]
> Lorem ipsum dolor sit amet

> [!quote] Optional Title
> Lorem ipsum dolor sit amet

> [!example]
> Lorem ipsum dolor sit amet

> [!bug]
> Lorem ipsum dolor sit amet

> [!danger]
> Lorem ipsum dolor sit amet

> [!failure]
> Lorem ipsum dolor sit amet

> [!warning]
> Lorem ipsum dolor sit amet

> [!question]
> Lorem ipsum dolor sit amet

> [!success]
> Lorem ipsum dolor sit amet

> [!tip]
> Lorem ipsum dolor sit amet

> [!todo]
> Lorem ipsum dolor sit amet

> [!abstract]
> Lorem ipsum dolor sit amet

> [!note]
> Lorem ipsum dolor sit amet
```

#### Aliases

| Main     | Alias              |
| -------- | ------------------ |
| danger   | error              |
| failure  | fail, missing      |
| warning  | caution, attention |
| question | help, faq          |
| success  | check, done        |
| tip      | hint, important    |
| abstract | summary, tldr      |

### Fence Syntax

You can also use a custom fence syntax with `:::`:

```markdown
:::note
This is a note using fence syntax
:::

:::warning
This is a warning with fence syntax
:::

:::tip
Pro tip: You can use either syntax!
:::
```

### Custom Titles

For GitHub/Obsidian syntax, you can provide a custom title:

```markdown
> [!WARNING] Custom Warning Title
> This warning has a custom title instead of the default "Warning"
```

### Implementation Details

Admonitions are:

- Parsed during markdown preprocessing
- Converted to internal AST representation
- Rendered with themed colors from the active color scheme
- Displayed with Unicode icons (ⓘ, ⚠, ✓, etc.)
- Support nested markdown content (paragraphs, lists, code, etc.)
