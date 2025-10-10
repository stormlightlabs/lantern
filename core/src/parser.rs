use crate::error::Result;
use crate::metadata::Meta;
use crate::slide::*;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

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

/// Split markdown content on `---` separators
fn split_slides(markdown: &str) -> Vec<String> {
    let mut slides = Vec::new();
    let mut current = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed == "---" {
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
    let parser = Parser::new(&markdown);
    let mut blocks = Vec::new();
    let mut block_stack: Vec<BlockBuilder> = Vec::new();
    let mut current_style = TextStyle::default();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    block_stack.push(BlockBuilder::Heading {
                        level: level as u8,
                        spans: Vec::new(),
                        style: current_style.clone(),
                    });
                }
                Tag::Paragraph => {
                    block_stack.push(BlockBuilder::Paragraph {
                        spans: Vec::new(),
                        style: current_style.clone(),
                    });
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
                    block_stack.push(BlockBuilder::Code {
                        language,
                        code: String::new(),
                    });
                }
                Tag::List(first) => {
                    block_stack.push(BlockBuilder::List {
                        ordered: first.is_some(),
                        items: Vec::new(),
                        current_item: Vec::new(),
                        style: current_style.clone(),
                    });
                }
                Tag::BlockQuote(_) => {
                    block_stack.push(BlockBuilder::BlockQuote { blocks: Vec::new() });
                }
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
                        blocks.push(builder.build());
                    }
                }
                TagEnd::List(_) => {
                    if let Some(builder) = block_stack.pop() {
                        blocks.push(builder.build());
                    }
                }
                TagEnd::BlockQuote(_) => {
                    if let Some(builder) = block_stack.pop() {
                        blocks.push(builder.build());
                    }
                }
                TagEnd::Item => {
                    if let Some(BlockBuilder::List {
                        current_item, items, ..
                    }) = block_stack.last_mut()
                    {
                        if !current_item.is_empty() {
                            items.push(ListItem {
                                spans: current_item.drain(..).collect(),
                                nested: None,
                            });
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
                    builder.add_text(text.to_string());
                }
            }

            Event::Code(code) => {
                if let Some(builder) = block_stack.last_mut() {
                    builder.add_code_span(code.to_string());
                }
            }

            Event::SoftBreak | Event::HardBreak => {
                if let Some(builder) = block_stack.last_mut() {
                    builder.add_text(" ".to_string());
                }
            }

            Event::Rule => {
                blocks.push(Block::Rule);
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
        style: TextStyle,
    },
    Paragraph {
        spans: Vec<TextSpan>,
        style: TextStyle,
    },
    Code {
        language: Option<String>,
        code: String,
    },
    List {
        ordered: bool,
        items: Vec<ListItem>,
        current_item: Vec<TextSpan>,
        style: TextStyle,
    },
    BlockQuote {
        blocks: Vec<Block>,
    },
}

impl BlockBuilder {
    fn add_text(&mut self, text: String) {
        match self {
            Self::Heading { spans, style, .. } | Self::Paragraph { spans, style } => {
                if !text.is_empty() {
                    spans.push(TextSpan {
                        text,
                        style: style.clone(),
                    });
                }
            }
            Self::Code { code, .. } => {
                code.push_str(&text);
            }
            Self::List {
                current_item, style, ..
            } => {
                if !text.is_empty() {
                    current_item.push(TextSpan {
                        text,
                        style: style.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    fn add_code_span(&mut self, code: String) {
        match self {
            Self::Heading { spans, .. } | Self::Paragraph { spans, .. } => {
                spans.push(TextSpan {
                    text: code,
                    style: TextStyle {
                        code: true,
                        ..Default::default()
                    },
                });
            }
            Self::List { current_item, .. } => {
                current_item.push(TextSpan {
                    text: code,
                    style: TextStyle {
                        code: true,
                        ..Default::default()
                    },
                });
            }
            _ => {}
        }
    }

    fn build(self) -> Block {
        match self {
            Self::Heading { level, spans, .. } => Block::Heading { level, spans },
            Self::Paragraph { spans, .. } => Block::Paragraph { spans },
            Self::Code { language, code } => Block::Code(CodeBlock { language, code }),
            Self::List { ordered, items, .. } => Block::List(List { ordered, items }),
            Self::BlockQuote { blocks } => Block::BlockQuote { blocks },
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
}
