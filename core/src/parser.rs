use crate::error::Result;
use crate::metadata::Meta;
use crate::slide::*;
use pulldown_cmark::{Alignment as PulldownAlignment, Event, Options, Parser, Tag, TagEnd};

/// Parse markdown content into metadata and slides
///
/// Extracts frontmatter metadata, then splits content on `---` separators.
pub fn parse_slides_with_meta(markdown: &str) -> Result<(Meta, Vec<Slide>)> {
    let (meta, content) = Meta::extract_from_markdown(markdown)?;
    let slides = parse_slides(&content)?;
    Ok((meta, slides))
}

/// Parse markdown content into a vector of slides
pub fn parse_slides(markdown: &str) -> Result<Vec<Slide>> {
    let sections = split_slides(markdown);
    sections.into_iter().map(parse_slide).collect()
}

/// Preprocess markdown to convert admonition syntax to a format we can parse
///
/// Converts both GitHub/Obsidian syntax (`> [!NOTE]`) and fence syntax (`:::note`)
/// into a special HTML-like format that we can detect in the event stream
fn preprocess_admonitions(markdown: &str) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if let Some(admonition_type) = parse_fence_admonition(trimmed) {
            result.push_str(&format!("<admonition type=\"{admonition_type}\">\n"));
            i += 1;
            while i < lines.len() {
                let content_line = lines[i];
                if content_line.trim() == ":::" {
                    result.push_str("</admonition>\n");
                    i += 1;
                    break;
                }
                result.push_str(content_line);
                result.push('\n');
                i += 1;
            }
            continue;
        }

        if trimmed.starts_with('>') {
            if let Some((admonition_type, title)) = parse_blockquote_admonition(trimmed) {
                result.push_str(&format!("<admonition type=\"{admonition_type}\""));
                if let Some(t) = title {
                    result.push_str(&format!(" title=\"{t}\""));
                }
                result.push_str(">\n");
                i += 1;

                while i < lines.len() {
                    let next_line = lines[i];
                    let next_trimmed = next_line.trim();
                    if next_trimmed.starts_with('>') {
                        let content = next_trimmed.strip_prefix('>').unwrap_or("").trim();
                        if !content.is_empty() {
                            result.push_str(content);
                            result.push('\n');
                        }
                        i += 1;
                    } else {
                        break;
                    }
                }
                result.push_str("</admonition>\n");
                continue;
            }
        }

        result.push_str(line);
        result.push('\n');
        i += 1;
    }

    result
}

/// Parse fence-style admonition: `:::note` or `:::warning Title`
fn parse_fence_admonition(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with(":::") {
        return None;
    }

    let content = trimmed.strip_prefix(":::").unwrap_or("").trim();
    if content.is_empty() {
        return None;
    }

    let parts: Vec<&str> = content.splitn(2, ' ').collect();
    let admonition_type = parts[0].to_lowercase();

    if admonition_type.is_empty() { None } else { Some(admonition_type) }
}

/// Parse blockquote-style admonition: `> [!NOTE]` or `> [!TIP] Custom Title`
fn parse_blockquote_admonition(line: &str) -> Option<(String, Option<String>)> {
    let content = line.trim().strip_prefix('>')?.trim();

    if !content.starts_with("[!") {
        return None;
    }

    let rest = content.strip_prefix("[!")?;
    let close_bracket = rest.find(']')?;
    let admonition_type = rest[..close_bracket].to_lowercase();

    let title = rest[close_bracket + 1..].trim();
    let title = if title.is_empty() { None } else { Some(title.to_string()) };

    Some((admonition_type, title))
}

/// Parse HTML admonition tag: `<admonition type="note" title="Title">`
fn parse_admonition_html_start(html: &str) -> Option<(AdmonitionType, Option<String>)> {
    let html = html.trim();
    if !html.starts_with("<admonition") {
        return None;
    }

    let type_start = html.find("type=\"")?;
    let type_value_start = type_start + 6;
    let type_end = html[type_value_start..].find('"')? + type_value_start;
    let admonition_type_str = &html[type_value_start..type_end];
    let admonition_type = admonition_type_str.parse().ok()?;

    let title = if let Some(title_start) = html.find("title=\"") {
        let title_value_start = title_start + 7;
        let title_end = html[title_value_start..].find('"')? + title_value_start;
        Some(html[title_value_start..title_end].to_string())
    } else {
        None
    };

    Some((admonition_type, title))
}

/// Split markdown content on `---` separators
///
/// Ignores `---` inside fenced code blocks to avoid incorrect slide splits
fn split_slides(markdown: &str) -> Vec<String> {
    let mut slides = Vec::new();
    let mut current = String::new();
    let mut in_code_block = false;

    for line in markdown.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
        }

        if trimmed == "---" && !in_code_block {
            if !current.trim().is_empty() {
                slides.push(current);
                current = String::new();
            }
        } else {
            current.push_str(line);
            current.push('\n');
        }
    }

    if !current.trim().is_empty() {
        slides.push(current);
    }

    slides
}

/// Parse a single slide from markdown
fn parse_slide(markdown: String) -> Result<Slide> {
    let preprocessed = preprocess_admonitions(&markdown);
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&preprocessed, options);
    let mut blocks = Vec::new();
    let mut block_stack: Vec<BlockBuilder> = Vec::new();
    let mut current_style = TextStyle::default();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    block_stack.push(BlockBuilder::Heading { level: level as u8, spans: Vec::new() });
                }
                Tag::Paragraph => {
                    block_stack.push(BlockBuilder::Paragraph { spans: Vec::new() });
                }
                Tag::CodeBlock(kind) => {
                    let language = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                            if lang.is_empty() {
                                None
                            } else {
                                Some(lang.to_string())
                            }
                        }
                        pulldown_cmark::CodeBlockKind::Indented => None,
                    };
                    block_stack.push(BlockBuilder::Code { language, code: String::new() });
                }
                Tag::List(first) => {
                    block_stack.push(BlockBuilder::List {
                        ordered: first.is_some(),
                        items: Vec::new(),
                        current_item: Vec::new(),
                    });
                }
                Tag::BlockQuote(_) => {
                    block_stack.push(BlockBuilder::BlockQuote { blocks: Vec::new() });
                }
                Tag::Table(alignments) => {
                    let converted_alignments = alignments
                        .iter()
                        .map(|a| match a {
                            PulldownAlignment::None | PulldownAlignment::Left => Alignment::Left,
                            PulldownAlignment::Center => Alignment::Center,
                            PulldownAlignment::Right => Alignment::Right,
                        })
                        .collect();
                    block_stack.push(BlockBuilder::Table {
                        headers: Vec::new(),
                        rows: Vec::new(),
                        current_row: Vec::new(),
                        current_cell: Vec::new(),
                        alignments: converted_alignments,
                        in_header: false,
                    });
                }
                Tag::TableHead => {
                    if let Some(BlockBuilder::Table { in_header, .. }) = block_stack.last_mut() {
                        *in_header = true;
                    }
                }
                Tag::TableRow => {}
                Tag::TableCell => {}
                Tag::Item => {}
                Tag::Emphasis => {
                    current_style.italic = true;
                }
                Tag::Strong => {
                    current_style.bold = true;
                }
                Tag::Strikethrough => {
                    current_style.strikethrough = true;
                }
                _ => {}
            },

            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) | TagEnd::Paragraph | TagEnd::CodeBlock => {
                    if let Some(builder) = block_stack.pop() {
                        let block = builder.build();
                        if let Some(BlockBuilder::Admonition { blocks: adm_blocks, .. }) = block_stack.last_mut() {
                            adm_blocks.push(block);
                        } else {
                            blocks.push(block);
                        }
                    }
                }
                TagEnd::List(_) => {
                    if let Some(builder) = block_stack.pop() {
                        let block = builder.build();
                        if let Some(BlockBuilder::Admonition { blocks: adm_blocks, .. }) = block_stack.last_mut() {
                            adm_blocks.push(block);
                        } else {
                            blocks.push(block);
                        }
                    }
                }
                TagEnd::BlockQuote(_) => {
                    if let Some(builder) = block_stack.pop() {
                        let block = builder.build();
                        if let Some(BlockBuilder::Admonition { blocks: adm_blocks, .. }) = block_stack.last_mut() {
                            adm_blocks.push(block);
                        } else {
                            blocks.push(block);
                        }
                    }
                }
                TagEnd::Table => {
                    if let Some(builder) = block_stack.pop() {
                        let block = builder.build();
                        if let Some(BlockBuilder::Admonition { blocks: adm_blocks, .. }) = block_stack.last_mut() {
                            adm_blocks.push(block);
                        } else {
                            blocks.push(block);
                        }
                    }
                }
                TagEnd::TableHead => {
                    if let Some(BlockBuilder::Table { current_row, headers, in_header, .. }) = block_stack.last_mut() {
                        if !current_row.is_empty() {
                            *headers = std::mem::take(current_row);
                        }
                        *in_header = false;
                    }
                }
                TagEnd::TableRow => {
                    if let Some(BlockBuilder::Table { current_row, rows, .. }) = block_stack.last_mut() {
                        if !current_row.is_empty() {
                            rows.push(std::mem::take(current_row));
                        }
                    }
                }
                TagEnd::TableCell => {
                    if let Some(BlockBuilder::Table { current_cell, current_row, .. }) = block_stack.last_mut() {
                        current_row.push(std::mem::take(current_cell));
                    }
                }
                TagEnd::Item => {
                    if let Some(BlockBuilder::List { current_item, items, .. }) = block_stack.last_mut() {
                        if !current_item.is_empty() {
                            items.push(ListItem { spans: std::mem::take(current_item), nested: None });
                        }
                    }
                }
                TagEnd::Emphasis => {
                    current_style.italic = false;
                }
                TagEnd::Strong => {
                    current_style.bold = false;
                }
                TagEnd::Strikethrough => {
                    current_style.strikethrough = false;
                }
                _ => {}
            },

            Event::Text(text) => {
                if let Some(builder) = block_stack.last_mut() {
                    builder.add_text(text.to_string(), &current_style);
                }
            }

            Event::Code(code) => {
                if let Some(builder) = block_stack.last_mut() {
                    builder.add_code_span(code.to_string());
                }
            }

            Event::SoftBreak | Event::HardBreak => {
                if let Some(builder) = block_stack.last_mut() {
                    builder.add_text(" ".to_string(), &current_style);
                }
            }

            Event::Rule => {
                blocks.push(Block::Rule);
            }

            Event::Html(html) => {
                if let Some((admonition_type, title)) = parse_admonition_html_start(&html) {
                    block_stack.push(BlockBuilder::Admonition { admonition_type, title, blocks: Vec::new() });
                } else if html.trim().starts_with("</admonition>") {
                    if let Some(builder) = block_stack.pop() {
                        blocks.push(builder.build());
                    }
                } else if !block_stack.is_empty() {
                    if let Some(BlockBuilder::Admonition { blocks: adm_blocks, .. }) = block_stack.last_mut() {
                        let inner_markdown = html.to_string();
                        let inner_options = Options::empty();
                        let inner_parser = Parser::new_ext(&inner_markdown, inner_options);
                        let mut inner_block_stack: Vec<BlockBuilder> = Vec::new();
                        let inner_style = TextStyle::default();

                        for inner_event in inner_parser {
                            match inner_event {
                                Event::Start(Tag::Paragraph) => {
                                    inner_block_stack.push(BlockBuilder::Paragraph { spans: Vec::new() });
                                }
                                Event::Text(text) => {
                                    if let Some(builder) = inner_block_stack.last_mut() {
                                        builder.add_text(text.to_string(), &inner_style);
                                    }
                                }
                                Event::End(TagEnd::Paragraph) => {
                                    if let Some(builder) = inner_block_stack.pop() {
                                        adm_blocks.push(builder.build());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            _ => {}
        }
    }

    Ok(Slide::with_blocks(blocks))
}

/// Helper to build blocks while parsing
enum BlockBuilder {
    Heading {
        level: u8,
        spans: Vec<TextSpan>,
    },
    Paragraph {
        spans: Vec<TextSpan>,
    },
    Code {
        language: Option<String>,
        code: String,
    },
    List {
        ordered: bool,
        items: Vec<ListItem>,
        current_item: Vec<TextSpan>,
    },
    BlockQuote {
        blocks: Vec<Block>,
    },
    Table {
        headers: Vec<Vec<TextSpan>>,
        rows: Vec<Vec<Vec<TextSpan>>>,
        current_row: Vec<Vec<TextSpan>>,
        current_cell: Vec<TextSpan>,
        alignments: Vec<Alignment>,
        in_header: bool,
    },
    Admonition {
        admonition_type: AdmonitionType,
        title: Option<String>,
        blocks: Vec<Block>,
    },
}

impl BlockBuilder {
    fn add_text(&mut self, text: String, current_style: &TextStyle) {
        match self {
            Self::Heading { spans, .. } | Self::Paragraph { spans, .. } => {
                if !text.is_empty() {
                    spans.push(TextSpan { text, style: current_style.clone() });
                }
            }
            Self::Code { code, .. } => {
                code.push_str(&text);
            }
            Self::List { current_item, .. } => {
                if !text.is_empty() {
                    current_item.push(TextSpan { text, style: current_style.clone() });
                }
            }
            Self::Table { current_cell, .. } => {
                if !text.is_empty() {
                    current_cell.push(TextSpan { text, style: current_style.clone() });
                }
            }
            Self::Admonition { .. } => {}
            _ => {}
        }
    }

    fn add_code_span(&mut self, code: String) {
        match self {
            Self::Heading { spans, .. } | Self::Paragraph { spans, .. } => {
                spans.push(TextSpan { text: code, style: TextStyle { code: true, ..Default::default() } });
            }
            Self::List { current_item, .. } => {
                current_item.push(TextSpan { text: code, style: TextStyle { code: true, ..Default::default() } });
            }
            Self::Table { current_cell, .. } => {
                current_cell.push(TextSpan { text: code, style: TextStyle { code: true, ..Default::default() } });
            }
            Self::Admonition { .. } => {}
            _ => {}
        }
    }

    fn build(self) -> Block {
        match self {
            Self::Heading { level, spans } => Block::Heading { level, spans },
            Self::Paragraph { spans } => Block::Paragraph { spans },
            Self::Code { language, code } => Block::Code(CodeBlock { language, code }),
            Self::List { ordered, items, .. } => Block::List(List { ordered, items }),
            Self::BlockQuote { blocks } => Block::BlockQuote { blocks },
            Self::Table { headers, rows, alignments, .. } => Block::Table(Table { headers, rows, alignments }),
            Self::Admonition { admonition_type, title, blocks } => {
                Block::Admonition(Admonition { admonition_type, title, blocks })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_slides_basic() {
        let markdown = "# Slide 1\n---\n# Slide 2";
        let slides = split_slides(markdown);
        assert_eq!(slides.len(), 2);
        assert!(slides[0].contains("Slide 1"));
        assert!(slides[1].contains("Slide 2"));
    }

    #[test]
    fn split_slides_empty() {
        let markdown = "";
        let slides = split_slides(markdown);
        assert_eq!(slides.len(), 0);
    }

    #[test]
    fn split_slides_single() {
        let markdown = "# Only Slide";
        let slides = split_slides(markdown);
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn split_slides_ignores_separator_in_code_block() {
        let markdown = r#"# Slide 1

```markdown
---
```

Content after code block

---

# Slide 2"#;
        let slides = split_slides(markdown);
        assert_eq!(slides.len(), 2);
        assert!(slides[0].contains("Slide 1"));
        assert!(slides[0].contains("---"));
        assert!(slides[0].contains("Content after code block"));
        assert!(slides[1].contains("Slide 2"));
    }

    #[test]
    fn parse_heading() {
        let slides = parse_slides("# Hello World").unwrap();
        assert_eq!(slides.len(), 1);

        match &slides[0].blocks[0] {
            Block::Heading { level, spans } => {
                assert_eq!(*level, 1);
                assert_eq!(spans[0].text, "Hello World");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn parse_paragraph() {
        let slides = parse_slides("This is a paragraph").unwrap();
        assert_eq!(slides.len(), 1);

        match &slides[0].blocks[0] {
            Block::Paragraph { spans } => {
                assert_eq!(spans[0].text, "This is a paragraph");
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn parse_code_block() {
        let markdown = "```rust\nfn main() {}\n```";
        let slides = parse_slides(markdown).unwrap();

        match &slides[0].blocks[0] {
            Block::Code(code) => {
                assert_eq!(code.language, Some("rust".to_string()));
                assert!(code.code.contains("fn main()"));
            }
            _ => panic!("Expected code block"),
        }
    }

    #[test]
    fn parse_list() {
        let markdown = "- Item 1\n- Item 2";
        let slides = parse_slides(markdown).unwrap();

        match &slides[0].blocks[0] {
            Block::List(list) => {
                assert!(!list.ordered);
                assert_eq!(list.items.len(), 2);
                assert_eq!(list.items[0].spans[0].text, "Item 1");
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn parse_multiple_slides() {
        let markdown = "# Slide 1\nContent 1\n---\n# Slide 2\nContent 2";
        let slides = parse_slides(markdown).unwrap();
        assert_eq!(slides.len(), 2);
    }

    #[test]
    fn parse_with_yaml_metadata() {
        let markdown = r#"---
theme: dark
author: Test Author
---
# First Slide
Content here
---
# Second Slide
More content"#;

        let (meta, slides) = parse_slides_with_meta(markdown).unwrap();
        assert_eq!(meta.theme, "dark");
        assert_eq!(meta.author, "Test Author");
        assert_eq!(slides.len(), 2);
    }

    #[test]
    fn parse_with_toml_metadata() {
        let markdown = r#"+++
theme = "monokai"
author = "Jane Doe"
+++
# Slide One
Test content"#;

        let (meta, slides) = parse_slides_with_meta(markdown).unwrap();
        assert_eq!(meta.theme, "monokai");
        assert_eq!(meta.author, "Jane Doe");
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn parse_without_metadata() {
        let markdown = "# Slide\nContent";
        let (meta, slides) = parse_slides_with_meta(markdown).unwrap();
        assert_eq!(meta, Meta::default());
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn parse_table() {
        let markdown = r#"| Name | Age |
| ---- | --- |
| Alice | 30 |
| Bob | 25 |"#;
        let slides = parse_slides(markdown).unwrap();
        assert_eq!(slides.len(), 1);

        match &slides[0].blocks[0] {
            Block::Table(table) => {
                assert_eq!(table.headers.len(), 2);
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.headers[0][0].text, "Name");
                assert_eq!(table.headers[1][0].text, "Age");
                assert_eq!(table.rows[0][0][0].text, "Alice");
                assert_eq!(table.rows[0][1][0].text, "30");
                assert_eq!(table.rows[1][0][0].text, "Bob");
                assert_eq!(table.rows[1][1][0].text, "25");
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn parse_table_with_alignment() {
        let markdown = r#"| Left | Center | Right |
| :--- | :----: | ----: |
| A | B | C |"#;
        let slides = parse_slides(markdown).unwrap();

        match &slides[0].blocks[0] {
            Block::Table(table) => {
                assert_eq!(table.alignments.len(), 3);
                assert!(matches!(table.alignments[0], Alignment::Left));
                assert!(matches!(table.alignments[1], Alignment::Center));
                assert!(matches!(table.alignments[2], Alignment::Right));
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn parse_table_with_styled_text() {
        let markdown = r#"| Name | Status |
| ---- | ------ |
| **Bold** | `code` |"#;
        let slides = parse_slides(markdown).unwrap();

        match &slides[0].blocks[0] {
            Block::Table(table) => {
                assert!(table.rows[0][0][0].style.bold);
                assert!(table.rows[0][1][0].style.code);
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn preprocess_github_admonition() {
        let markdown = r#"> [!NOTE]
> This is a note"#;
        let preprocessed = preprocess_admonitions(markdown);
        assert!(preprocessed.contains("<admonition"));
        assert!(preprocessed.contains("type=\"note\""));
        assert!(preprocessed.contains("</admonition>"));

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&preprocessed, options);
        let events: Vec<_> = parser.collect();

        let has_html = events.iter().any(|e| matches!(e, Event::Html(_)));
        assert!(has_html, "Should have HTML events");
    }

    #[test]
    fn parse_github_admonition_note() {
        let markdown = r#"> [!NOTE]
> This is a note"#;
        let slides = parse_slides(markdown).unwrap();
        assert_eq!(slides.len(), 1);

        match &slides[0].blocks[0] {
            Block::Admonition(admonition) => {
                assert_eq!(admonition.admonition_type, AdmonitionType::Note);
                assert_eq!(admonition.title, None);
                assert_eq!(admonition.blocks.len(), 1);
            }
            _ => panic!("Expected admonition, got: {:?}", slides[0].blocks[0]),
        }
    }

    #[test]
    fn parse_github_admonition_with_title() {
        let markdown = r#"> [!WARNING] Custom Warning
> Be careful!"#;
        let slides = parse_slides(markdown).unwrap();

        match &slides[0].blocks[0] {
            Block::Admonition(admonition) => {
                assert_eq!(admonition.admonition_type, AdmonitionType::Warning);
                assert_eq!(admonition.title, Some("Custom Warning".to_string()));
                assert_eq!(admonition.blocks.len(), 1);
            }
            _ => panic!("Expected admonition"),
        }
    }

    #[test]
    fn parse_fence_admonition() {
        let markdown = r#":::tip
This is a helpful tip
:::"#;
        let slides = parse_slides(markdown).unwrap();
        assert_eq!(slides.len(), 1);

        match &slides[0].blocks[0] {
            Block::Admonition(admonition) => {
                assert_eq!(admonition.admonition_type, AdmonitionType::Tip);
                assert_eq!(admonition.blocks.len(), 1);
            }
            _ => panic!("Expected admonition"),
        }
    }

    #[test]
    fn parse_admonition_danger_alias() {
        let markdown = r#"> [!DANGER]
> Dangerous content"#;
        let slides = parse_slides(markdown).unwrap();

        match &slides[0].blocks[0] {
            Block::Admonition(admonition) => {
                assert_eq!(admonition.admonition_type, AdmonitionType::Danger);
            }
            _ => panic!("Expected admonition"),
        }
    }

    #[test]
    fn admonition_type_from_str_note() {
        assert_eq!("note".parse::<AdmonitionType>(), Ok(AdmonitionType::Note));
        assert_eq!("NOTE".parse::<AdmonitionType>(), Ok(AdmonitionType::Note));
    }

    #[test]
    fn admonition_type_from_str_tip_alias() {
        assert_eq!("tip".parse::<AdmonitionType>(), Ok(AdmonitionType::Tip));
        assert_eq!("hint".parse::<AdmonitionType>(), Ok(AdmonitionType::Tip));
    }

    #[test]
    fn admonition_type_from_str_warning_aliases() {
        assert_eq!("warning".parse::<AdmonitionType>(), Ok(AdmonitionType::Warning));
        assert_eq!("caution".parse::<AdmonitionType>(), Ok(AdmonitionType::Warning));
        assert_eq!("attention".parse::<AdmonitionType>(), Ok(AdmonitionType::Warning));
    }

    #[test]
    fn admonition_type_from_str_invalid() {
        assert!("invalid".parse::<AdmonitionType>().is_err());
        assert!("".parse::<AdmonitionType>().is_err());
    }
}
