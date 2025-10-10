use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
};
use slides_core::{
    slide::{Block, CodeBlock, List, Table, TextSpan, TextStyle},
    theme::ThemeColors,
};

/// Render a slide's blocks into ratatui Text
///
/// Converts slide blocks into styled ratatui text with theming applied.
pub fn render_slide_content(blocks: &[Block], theme: &ThemeColors) -> Text<'static> {
    let mut lines = Vec::new();

    for block in blocks {
        match block {
            Block::Heading { level, spans } => render_heading(*level, spans, theme, &mut lines),
            Block::Paragraph { spans } => render_paragraph(spans, theme, &mut lines),
            Block::Code(code_block) => render_code_block(code_block, theme, &mut lines),
            Block::List(list) => render_list(list, theme, &mut lines, 0),
            Block::Rule => render_rule(theme, &mut lines),
            Block::BlockQuote { blocks } => render_blockquote(blocks, theme, &mut lines),
            Block::Table(table) => render_table(table, theme, &mut lines),
        }

        lines.push(Line::raw(""));
    }

    Text::from(lines)
}

/// Get heading prefix
fn get_prefix(level: u8) -> &'static str {
    match level {
        1 => "# ",
        2 => "## ",
        3 => "### ",
        4 => "#### ",
        5 => "##### ",
        _ => "###### ",
    }
}

/// Render a heading with size based on level
fn render_heading(level: u8, spans: &[TextSpan], theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let prefix = get_prefix(level);
    let heading_style = to_ratatui_style(&theme.heading);
    let mut line_spans = vec![Span::styled(prefix.to_string(), heading_style)];

    for span in spans {
        line_spans.push(create_span(span, theme, true));
    }

    lines.push(Line::from(line_spans));
}

/// Render a paragraph with styled text spans
fn render_paragraph(spans: &[TextSpan], theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let line_spans: Vec<_> = spans.iter().map(|span| create_span(span, theme, false)).collect();
    lines.push(Line::from(line_spans));
}

/// Render a code block with monospace styling
fn render_code_block(code: &CodeBlock, theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let fence_style = to_ratatui_style(&theme.code_fence);
    let code_style = to_ratatui_style(&theme.code);

    if let Some(lang) = &code.language {
        lines.push(Line::from(Span::styled(format!("```{}", lang), fence_style)));
    } else {
        lines.push(Line::from(Span::styled("```".to_string(), fence_style)));
    }

    for line in code.code.lines() {
        lines.push(Line::from(Span::styled(line.to_string(), code_style)));
    }

    lines.push(Line::from(Span::styled("```".to_string(), fence_style)));
}

/// Render a list with bullets or numbers
fn render_list(list: &List, theme: &ThemeColors, lines: &mut Vec<Line<'static>>, indent: usize) {
    let marker_style = to_ratatui_style(&theme.list_marker);

    for (idx, item) in list.items.iter().enumerate() {
        let prefix = if list.ordered {
            format!("{}{}. ", "  ".repeat(indent), idx + 1)
        } else {
            format!("{}• ", "  ".repeat(indent))
        };

        let mut line_spans = vec![Span::styled(prefix, marker_style)];

        for span in &item.spans {
            line_spans.push(create_span(span, theme, false));
        }

        lines.push(Line::from(line_spans));

        if let Some(nested) = &item.nested {
            render_list(nested, theme, lines, indent + 1);
        }
    }
}

/// Render a horizontal rule
fn render_rule(theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let rule_style = to_ratatui_style(&theme.rule);
    let rule = "─".repeat(60);
    lines.push(Line::from(Span::styled(rule, rule_style)));
}

/// Render a blockquote with indentation
fn render_blockquote(blocks: &[Block], theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let border_style = to_ratatui_style(&theme.blockquote_border);

    for block in blocks {
        match block {
            Block::Paragraph { spans } => {
                let mut line_spans = vec![Span::styled("│ ".to_string(), border_style)];

                for span in spans {
                    line_spans.push(create_span(span, theme, false));
                }

                lines.push(Line::from(line_spans));
            }
            _ => {}
        }
    }
}

/// Render a table with basic formatting
fn render_table(table: &Table, theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let border_style = to_ratatui_style(&theme.table_border);

    if !table.headers.is_empty() {
        let mut header_line = Vec::new();
        for (idx, header) in table.headers.iter().enumerate() {
            if idx > 0 {
                header_line.push(Span::styled(" │ ".to_string(), border_style));
            }
            for span in header {
                header_line.push(create_span(span, theme, true));
            }
        }
        lines.push(Line::from(header_line));

        let separator = "─".repeat(60);
        lines.push(Line::from(Span::styled(separator, border_style)));
    }

    for row in &table.rows {
        let mut row_line = Vec::new();
        for (idx, cell) in row.iter().enumerate() {
            if idx > 0 {
                row_line.push(Span::styled(" │ ".to_string(), border_style));
            }
            for span in cell {
                row_line.push(create_span(span, theme, false));
            }
        }
        lines.push(Line::from(row_line));
    }
}

/// Create a styled span from a TextSpan
fn create_span(text_span: &TextSpan, theme: &ThemeColors, is_heading: bool) -> Span<'static> {
    let style = apply_theme_style(theme, &text_span.style, is_heading);
    Span::styled(text_span.text.clone(), style)
}

/// Apply theme colors and text styling
fn apply_theme_style(theme: &ThemeColors, text_style: &TextStyle, is_heading: bool) -> Style {
    let mut style = if is_heading {
        to_ratatui_style(&theme.heading)
    } else if text_style.code {
        to_ratatui_style(&theme.code)
    } else {
        to_ratatui_style(&theme.body)
    };

    if text_style.bold {
        style = style.add_modifier(Modifier::BOLD);
    }
    if text_style.italic {
        style = style.add_modifier(Modifier::ITALIC);
    }
    if text_style.strikethrough {
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }

    style
}

/// Convert owo-colors Style to ratatui Style
///
/// Since owo-colors Style is opaque, we return a default ratatui style.
/// The theme provides semantic meaning; actual visual styling is defined here.
fn to_ratatui_style(_owo_style: &owo_colors::Style) -> Style {
    Style::default()
}

#[cfg(test)]
mod tests {
    use slides_core::slide::ListItem;

    use super::*;

    #[test]
    fn render_heading_basic() {
        let blocks = vec![Block::Heading { level: 1, spans: vec![TextSpan::plain("Test Heading")] }];
        let theme = ThemeColors::default();
        let text = render_slide_content(&blocks, &theme);
        assert!(!text.lines.is_empty());
    }

    #[test]
    fn render_paragraph_basic() {
        let blocks = vec![Block::Paragraph { spans: vec![TextSpan::plain("Test paragraph")] }];
        let theme = ThemeColors::default();
        let text = render_slide_content(&blocks, &theme);
        assert!(!text.lines.is_empty());
    }

    #[test]
    fn render_code_block() {
        let blocks = vec![Block::Code(CodeBlock::with_language("rust", "fn main() {}"))];
        let theme = ThemeColors::default();
        let text = render_slide_content(&blocks, &theme);
        assert!(text.lines.len() > 2);
    }

    #[test]
    fn render_list_unordered() {
        let list = List {
            ordered: false,
            items: vec![
                ListItem { spans: vec![TextSpan::plain("Item 1")], nested: None },
                ListItem { spans: vec![TextSpan::plain("Item 2")], nested: None },
            ],
        };
        let blocks = vec![Block::List(list)];
        let theme = ThemeColors::default();
        let text = render_slide_content(&blocks, &theme);
        assert!(text.lines.len() >= 2);
    }

    #[test]
    fn render_styled_text() {
        let blocks = vec![Block::Paragraph {
            spans: vec![
                TextSpan::bold("Bold"),
                TextSpan::plain(" "),
                TextSpan::italic("Italic"),
                TextSpan::plain(" "),
                TextSpan::code("code"),
            ],
        }];
        let theme = ThemeColors::default();
        let text = render_slide_content(&blocks, &theme);
        assert!(!text.lines.is_empty());
    }
}
