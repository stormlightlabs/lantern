---
theme: default
author: Learn Markdown
---

<!-- markdownlint-disable MD034 MD035 -->

# Markdown Basics

A quick reference for Markdown syntax

---

## Headings

Markdown supports multiple heading styles:

```markdown
# This is an h1
## This is an h2
### This is an h3
#### This is an h4
##### This is an h5
###### This is an h6
```

Alternative syntax for h1 and h2:

```markdown
This is an h1
=============

This is an h2
-------------
```

---

## Text Formatting

**Bold text:**

```markdown
**This text is in bold.**
__And so is this text.__
```

*Italic text:*

```markdown
*This text is in italics.*
_And so is this text._
```

Combined:

```markdown
***This text is in both.***
**_As is this!_**
*__And this!__*
```

Strikethrough:

```markdown
~~This text is rendered with strikethrough.~~
```

---

## Paragraphs

Paragraphs are separated by blank lines:

```markdown
This is a paragraph. I'm typing in a paragraph.

Now I'm in paragraph 2.
I'm still in paragraph 2 too!

I'm in paragraph three!
```

Line breaks require two spaces at the end or `<br />`:

```markdown
I end with two spaces (highlight to see them).
There's a <br /> above me!
```

---

## Block Quotes

Use `>` to create block quotes:

```markdown
> This is a block quote. You can either
> manually wrap your lines and put a `>`
> before every line or you can let your
> lines get really long and wrap on their own.
```

Nested quotes:

```markdown
> You can also use more than one level
>> of indentation?
> How neat is that?
```

---

## Lists

**Unordered lists** use `*`, `+`, or `-`:

```markdown
* Item
* Item
* Another item

- Item
- Item
- One last item
```

**Ordered lists** use numbers:

```markdown
1. Item one
2. Item two
3. Item three
```

Nested lists:

```markdown
1. Item one
2. Item two
3. Item three
    * Sub-item
    * Sub-item
4. Item four
```

---

## Task Lists

Create checkboxes with `[ ]` and `[x]`:

```markdown
- [ ] First task to complete
- [ ] Second task that needs done
- [x] This task has been completed
```

> [!NOTE]
> Task lists are a GitHub-flavored Markdown extension

---

## Code

**Inline code** uses backticks:

```markdown
John didn't even know what the `go_to()` function did!
```

**Code blocks** use triple backticks or indentation:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```

    This is code
    So is this
````

---

## Horizontal Rules

Create horizontal rules with three or more:

```markdown
***
---
- - -
****************
```

All render as:

***

___

- - -

---

## Links

**Inline links:**

```markdown
[Click me!](http://test.com/)
[Click me!](http://test.com/ "Link to Test.com")
[Go to music](/music/)
```

**Reference links:**

```markdown
[Click this link][link1] for more info!
[Also check out this link][foobar] if you want.

[link1]: http://test.com/ "Cool!"
[foobar]: http://foobar.biz/ "Alright!"
```

**Implicit reference:**

```markdown
[This][] is a link.

[This]: http://thisisalink.com/
```

---

## Internal Links

Link to headings using slugified IDs:

```markdown
- [Heading](#heading)
- [Another heading](#another-heading)
- [Chapter](#chapter)
  - [Subchapter <h3 />](#subchapter-h3-)
```

> [!TIP]
> Heading IDs are created by lowercasing and replacing spaces with hyphens

---

## Images

**Inline images:**

```markdown
![Alt text for image](http://imgur.com/myimage.jpg "Optional title")
```

**Reference images:**

```markdown
![This is the alt-attribute.][myimage]

[myimage]: relative/urls/cool/image.jpg "Optional title"
```

> [!NOTE]
> Images use the same syntax as links, but with a `!` prefix

---

## Automatic Links

URLs and email addresses can be auto-linked:

```markdown
<http://testwebsite.com/>
<foo@bar.com>
```

These are equivalent to:

```markdown
[http://testwebsite.com/](http://testwebsite.com/)
[foo@bar.com](mailto:foo@bar.com)
```

---

## Escaping

Use backslash to escape special characters:

```markdown
I want to type *this* but not in italics:
\*this text surrounded by asterisks\*
```

Special characters you can escape:

```markdown
\   backslash
`   backtick
*   asterisk
_   underscore
{}  curly braces
[]  square brackets
()  parentheses
#   hash mark
+   plus sign
-   minus sign
.   dot
!   exclamation mark
```

---

## HTML Elements

You can use HTML in Markdown:

```markdown
Your computer crashed? Try sending a
<kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>Del</kbd>
```

> [!WARNING]
> You cannot use Markdown syntax within HTML element contents

---

## Tables

Create tables with pipes and hyphens:

```markdown
| Col1         | Col2     | Col3          |
| :----------- | :------: | ------------: |
| Left-aligned | Centered | Right-aligned |
| blah         | blah     | blah          |
```

Compact syntax also works:

```markdown
Col 1 | Col2 | Col3
:-- | :-: | --:
Ugh this is ugly | make it | stop
```

Alignment is controlled by colons:

- `:--` = left-aligned
- `:-:` = centered
- `--:` = right-aligned

---

## Admonitions

> [!IMPORTANT]
> Admonitions are NOT standard Markdown - they are an extension

Common admonition types:

```markdown
> [!NOTE]
> Useful information

> [!TIP]
> Helpful advice

> [!IMPORTANT]
> Critical information

> [!WARNING]
> Proceed with caution

> [!CAUTION]
> Potential risks ahead
```

---

## Admonition Examples

> [!NOTE]
> This is a note admonition with helpful context

> [!TIP]
> Use Markdown for clear, readable documentation

> [!WARNING]
> Not all Markdown processors support the same features

> [!IMPORTANT]
> Always check your Markdown processor's documentation for supported features

---

## Comments

HTML comments work in Markdown:

```markdown
<!-- This is a comment and won't be rendered -->
```

Comments are useful for:

- Leaving notes for yourself or collaborators
- Temporarily hiding content
- Adding metadata that shouldn't display

---

## Summary

Markdown provides:

- **Simple syntax** for formatted text
- **Readable source** that looks good even as plain text
- **Portable format** supported by many tools
- **Extensions** like tables, task lists, and admonitions

> [!SUCCESS]
> You now know the basics of Markdown!

---

## Resources

**Learn more:**

- Markdown Guide (https://www.markdownguide.org/)
- GitHub Flavored Markdown (https://github.github.com/gfm/)
- CommonMark Spec (https://commonmark.org/)

**Practice:**

- Markdown Tutorial (https://www.markdowntutorial.com/)
- Dillinger (https://dillinger.io/) - Online Markdown editor

---

## Thank You

Happy writing!
