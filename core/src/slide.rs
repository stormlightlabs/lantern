use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// A single slide in a presentation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slide {
    /// The content blocks that make up this slide
    pub blocks: Vec<Block>,
    /// Optional speaker notes (not displayed on main slide)
    pub notes: Option<String>,
}

impl Slide {
    pub fn new() -> Self {
        Self { blocks: Vec::new(), notes: None }
    }

    pub fn with_blocks(blocks: Vec<Block>) -> Self {
        Self { blocks, notes: None }
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

impl Default for Slide {
    fn default() -> Self {
        Self::new()
    }
}

/// Content block types that can appear in a slide
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Block {
    /// Heading with level (1-6) and text spans
    Heading { level: u8, spans: Vec<TextSpan> },
    /// Paragraph of text spans
    Paragraph { spans: Vec<TextSpan> },
    /// Code block with optional language and content
    Code(CodeBlock),
    /// Ordered or unordered list
    List(List),
    /// Horizontal rule/divider
    Rule,
    /// Block quote
    BlockQuote { blocks: Vec<Block> },
    /// Table
    Table(Table),
    /// Admonition/alert box with type, optional title, and content
    Admonition(Admonition),
    /// Image with path and alt text
    Image { path: String, alt: String },
}

/// Styled text span within a block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSpan {
    pub text: String,
    pub style: TextStyle,
}

impl TextSpan {
    pub fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle::default() }
    }

    pub fn bold(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle { bold: true, ..Default::default() } }
    }

    pub fn italic(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle { italic: true, ..Default::default() } }
    }

    pub fn code(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle { code: true, ..Default::default() } }
    }
}

/// Text styling flags
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub code: bool,
}

/// Code block with language and content
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeBlock {
    /// Programming language for syntax highlighting
    pub language: Option<String>,
    /// Raw code content
    pub code: String,
}

impl CodeBlock {
    pub fn new(code: impl Into<String>) -> Self {
        Self { language: None, code: code.into() }
    }

    pub fn with_language(language: impl Into<String>, code: impl Into<String>) -> Self {
        Self { language: Some(language.into()), code: code.into() }
    }
}

/// List (ordered or unordered)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub ordered: bool,
    pub items: Vec<ListItem>,
}

/// Single list item that can contain blocks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListItem {
    pub spans: Vec<TextSpan>,
    pub nested: Option<Box<List>>,
}

/// Table with headers and rows
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Table {
    pub headers: Vec<Vec<TextSpan>>,
    pub rows: Vec<Vec<Vec<TextSpan>>>,
    pub alignments: Vec<Alignment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// Admonition type determines styling and icon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdmonitionType {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
    Danger,
    Error,
    Info,
    Success,
    Question,
    Example,
    Quote,
    Abstract,
    Todo,
    Bug,
    Failure,
}

/// Error type for parsing AdmonitionType
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAdmonitionTypeError;

impl std::fmt::Display for ParseAdmonitionTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid admonition type")
    }
}

impl std::error::Error for ParseAdmonitionTypeError {}

impl FromStr for AdmonitionType {
    type Err = ParseAdmonitionTypeError;

    /// Parse admonition type from string (case-insensitive)
    ///
    /// Supports GitHub and Obsidian aliases
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "note" => Ok(Self::Note),
            "tip" | "hint" => Ok(Self::Tip),
            "important" => Ok(Self::Important),
            "warning" | "caution" | "attention" => Ok(Self::Warning),
            "danger" | "error" => Ok(Self::Danger),
            "info" => Ok(Self::Info),
            "success" | "check" | "done" => Ok(Self::Success),
            "question" | "help" | "faq" => Ok(Self::Question),
            "example" => Ok(Self::Example),
            "quote" => Ok(Self::Quote),
            "abstract" | "summary" | "tldr" => Ok(Self::Abstract),
            "todo" => Ok(Self::Todo),
            "bug" => Ok(Self::Bug),
            "failure" | "fail" | "missing" => Ok(Self::Failure),
            _ => Err(ParseAdmonitionTypeError),
        }
    }
}

/// Admonition/alert box with styled content
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Admonition {
    pub admonition_type: AdmonitionType,
    pub title: Option<String>,
    pub blocks: Vec<Block>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slide_creation() {
        let slide = Slide::new();
        assert!(slide.is_empty());
        assert_eq!(slide.blocks.len(), 0);
    }

    #[test]
    fn slide_with_blocks() {
        let blocks = vec![Block::Paragraph { spans: vec![TextSpan::plain("Hello")] }];
        let slide = Slide::with_blocks(blocks.clone());
        assert!(!slide.is_empty());
        assert_eq!(slide.blocks.len(), 1);
    }

    #[test]
    fn text_span_styles() {
        let plain = TextSpan::plain("text");
        assert!(!plain.style.bold);
        assert!(!plain.style.italic);

        let bold = TextSpan::bold("text");
        assert!(bold.style.bold);

        let italic = TextSpan::italic("text");
        assert!(italic.style.italic);

        let code = TextSpan::code("text");
        assert!(code.style.code);
    }

    #[test]
    fn code_block_creation() {
        let code = CodeBlock::new("fn main() {}");
        assert_eq!(code.language, None);
        assert_eq!(code.code, "fn main() {}");

        let rust_code = CodeBlock::with_language("rust", "fn main() {}");
        assert_eq!(rust_code.language, Some("rust".to_string()));
    }
}
