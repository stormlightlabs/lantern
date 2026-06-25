//! Slide data model used by the parser, printer, validator, and TUI.

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
    /// Create an empty slide with no speaker notes.
    pub fn new() -> Self {
        Self { blocks: Vec::new(), notes: None }
    }

    /// Create a slide from existing content blocks.
    pub fn with_blocks(blocks: Vec<Block>) -> Self {
        Self { blocks, notes: None }
    }

    /// Return true when the slide contains no visible blocks.
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
    Heading {
        /// Markdown heading level from 1 through 6.
        level: u8,
        /// Styled heading text.
        spans: Vec<TextSpan>,
    },
    /// Paragraph of text spans
    Paragraph {
        /// Styled paragraph text.
        spans: Vec<TextSpan>,
    },
    /// Code block with optional language and content
    Code(CodeBlock),
    /// Ordered or unordered list
    List(List),
    /// Horizontal rule/divider
    Rule,
    /// Block quote
    BlockQuote {
        /// Nested blocks inside the quoted region.
        blocks: Vec<Block>,
    },
    /// Table
    Table(Table),
    /// Admonition/alert box with type, optional title, and content
    Admonition(Admonition),
    /// Image with path and alt text
    Image {
        /// Image path as written in the deck.
        path: String,
        /// Alternative text from the markdown image.
        alt: String,
    },
}

/// Styled text span within a block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSpan {
    /// Text content for this span.
    pub text: String,
    /// Inline style flags to apply to the text.
    pub style: TextStyle,
}

impl TextSpan {
    /// Create an unstyled text span.
    pub fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle::default() }
    }

    /// Create a bold text span.
    pub fn bold(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle { bold: true, ..Default::default() } }
    }

    /// Create an italic text span.
    pub fn italic(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle { italic: true, ..Default::default() } }
    }

    /// Create an inline-code text span.
    pub fn code(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: TextStyle { code: true, ..Default::default() } }
    }
}

/// Text styling flags
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextStyle {
    /// Whether the span should render in bold.
    pub bold: bool,
    /// Whether the span should render in italic.
    pub italic: bool,
    /// Whether the span should render with strikethrough.
    pub strikethrough: bool,
    /// Whether the span should render as inline code.
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
    /// Create a code block without a language hint.
    pub fn new(code: impl Into<String>) -> Self {
        Self { language: None, code: code.into() }
    }

    /// Create a code block with a language hint for syntax highlighting.
    pub fn with_language(language: impl Into<String>, code: impl Into<String>) -> Self {
        Self { language: Some(language.into()), code: code.into() }
    }
}

/// List (ordered or unordered)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    /// Whether this list is ordered instead of bulleted.
    pub ordered: bool,
    /// Top-level list items.
    pub items: Vec<ListItem>,
}

/// Single list item that can contain blocks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListItem {
    /// Styled text for this list item.
    pub spans: Vec<TextSpan>,
    /// Optional nested list below this item.
    pub nested: Option<Box<List>>,
}

/// Table with headers and rows
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Table {
    /// Header cells, each represented as styled text spans.
    pub headers: Vec<Vec<TextSpan>>,
    /// Body rows, cells, and styled text spans.
    pub rows: Vec<Vec<Vec<TextSpan>>>,
    /// Per-column alignment hints parsed from the markdown separator row.
    pub alignments: Vec<Alignment>,
}

/// Horizontal text alignment for markdown table columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    /// Left-align cell contents.
    Left,
    /// Center cell contents.
    Center,
    /// Right-align cell contents.
    Right,
}

/// Admonition type determines styling and icon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdmonitionType {
    /// General note callout.
    Note,
    /// Tip or hint callout.
    Tip,
    /// Important information callout.
    Important,
    /// Warning callout.
    Warning,
    /// Caution callout.
    Caution,
    /// Danger callout.
    Danger,
    /// Error callout.
    Error,
    /// Informational callout.
    Info,
    /// Success or done callout.
    Success,
    /// Question or FAQ callout.
    Question,
    /// Example callout.
    Example,
    /// Quote callout.
    Quote,
    /// Abstract, summary, or TLDR callout.
    Abstract,
    /// Todo callout.
    Todo,
    /// Bug callout.
    Bug,
    /// Failure or missing callout.
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
    /// Kind of admonition to render.
    pub admonition_type: AdmonitionType,
    /// Optional display title.
    pub title: Option<String>,
    /// Nested slide blocks inside the admonition.
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
