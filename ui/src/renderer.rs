use lantern_core::{
    highlighter,
    slide::{Block, CodeBlock, List, Table, TextSpan, TextStyle},
    theme::ThemeColors,
};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
};
use unicode_width::UnicodeWidthChar;

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
            Block::Admonition(admonition) => render_admonition(admonition, theme, &mut lines),
        }

        lines.push(Line::raw(""));
    }

    Text::from(lines)
}

/// Get heading prefix using Unicode block symbols
fn get_prefix(level: u8) -> &'static str {
    match level {
        1 => "▉ ", // Large block / heavy fill (U+2589)
        2 => "▓ ", // Dark shade (U+2593)
        3 => "▒ ", // Medium shade (U+2592)
        4 => "░ ", // Light shade (U+2591)
        5 => "▌ ", // Left half block (U+258C)
        _ => "▌ ", // Left half block (U+258C) for h6
    }
}

/// Render a heading with size based on level
fn render_heading(level: u8, spans: &[TextSpan], theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let prefix = get_prefix(level);
    let heading_style = to_ratatui_style(&theme.heading, theme.heading_bold);
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

/// Render a code block with syntax highlighting
fn render_code_block(code: &CodeBlock, theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let fence_style = to_ratatui_style(&theme.code_fence, false);

    if let Some(lang) = &code.language {
        lines.push(Line::from(Span::styled(format!("```{lang}"), fence_style)));
    } else {
        lines.push(Line::from(Span::styled("```".to_string(), fence_style)));
    }

    let highlighted_lines = highlighter::highlight_code(&code.code, code.language.as_deref(), theme);

    for tokens in highlighted_lines {
        let mut line_spans = Vec::new();
        for token in tokens {
            let token_style = to_ratatui_style(&token.color, false);
            line_spans.push(Span::styled(token.text, token_style));
        }
        lines.push(Line::from(line_spans));
    }

    lines.push(Line::from(Span::styled("```".to_string(), fence_style)));
}

/// Render a list with bullets or numbers
fn render_list(list: &List, theme: &ThemeColors, lines: &mut Vec<Line<'static>>, indent: usize) {
    let marker_style = to_ratatui_style(&theme.list_marker, false);

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
    let rule_style = to_ratatui_style(&theme.rule, false);
    let rule = "─".repeat(60);
    lines.push(Line::from(Span::styled(rule, rule_style)));
}

/// Render a blockquote with indentation
fn render_blockquote(blocks: &[Block], theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let border_style = to_ratatui_style(&theme.blockquote_border, false);

    for block in blocks {
        if let Block::Paragraph { spans } = block {
            let mut line_spans = vec![Span::styled("│ ".to_string(), border_style)];

            for span in spans {
                line_spans.push(create_span(span, theme, false));
            }

            lines.push(Line::from(line_spans));
        }
    }
}

/// Render an admonition with colored border and icon
fn render_admonition(
    admonition: &lantern_core::slide::Admonition, theme: &ThemeColors, lines: &mut Vec<Line<'static>>,
) {
    use lantern_core::slide::AdmonitionType;

    let (icon, color, default_title) = match admonition.admonition_type {
        AdmonitionType::Note => ("\u{24D8}", &theme.admonition_note, "Note"),
        AdmonitionType::Tip => ("\u{1F4A1}", &theme.admonition_tip, "Tip"),
        AdmonitionType::Important => ("\u{2757}", &theme.admonition_tip, "Important"),
        AdmonitionType::Warning => ("\u{26A0}", &theme.admonition_warning, "Warning"),
        AdmonitionType::Caution => ("\u{26A0}", &theme.admonition_warning, "Caution"),
        AdmonitionType::Danger => ("\u{26D4}", &theme.admonition_danger, "Danger"),
        AdmonitionType::Error => ("\u{2717}", &theme.admonition_danger, "Error"),
        AdmonitionType::Info => ("\u{24D8}", &theme.admonition_info, "Info"),
        AdmonitionType::Success => ("\u{2713}", &theme.admonition_success, "Success"),
        AdmonitionType::Question => ("?", &theme.admonition_info, "Question"),
        AdmonitionType::Example => ("\u{25B8}", &theme.admonition_success, "Example"),
        AdmonitionType::Quote => ("\u{201C}", &theme.admonition_info, "Quote"),
        AdmonitionType::Abstract => ("\u{00A7}", &theme.admonition_note, "Abstract"),
        AdmonitionType::Todo => ("\u{2610}", &theme.admonition_info, "Todo"),
        AdmonitionType::Bug => ("\u{1F41B}", &theme.admonition_danger, "Bug"),
        AdmonitionType::Failure => ("\u{2717}", &theme.admonition_danger, "Failure"),
    };

    let title = admonition.title.as_deref().unwrap_or(default_title);
    let color_style = to_ratatui_style(color, false);
    let bold_color_style = to_ratatui_style(color, true);

    let top_border = format!("\u{256D}{}\u{256E}", "\u{2500}".repeat(58));
    lines.push(Line::from(Span::styled(top_border, color_style)));

    let icon_display_width = icon.chars().next().and_then(|c| c.width()).unwrap_or(1);

    let title_line = vec![
        Span::styled("\u{2502} ".to_string(), color_style),
        Span::raw(format!("{icon} ")),
        Span::styled(title.to_string(), bold_color_style),
        Span::styled(
            " ".repeat(56_usize.saturating_sub(icon_display_width + 1 + title.len())),
            color_style,
        ),
        Span::styled(" \u{2502}".to_string(), color_style),
    ];
    lines.push(Line::from(title_line));

    if !admonition.blocks.is_empty() {
        let separator = format!("\u{251C}{}\u{2524}", "\u{2500}".repeat(58));
        lines.push(Line::from(Span::styled(separator, color_style)));

        for block in &admonition.blocks {
            if let Block::Paragraph { spans } = block {
                let text: String = spans.iter().map(|s| s.text.as_str()).collect();
                let words: Vec<&str> = text.split_whitespace().collect();
                let content_width = 56; // 60 total - 2 for borders - 2 for spaces

                let mut current_line = String::new();
                for word in words {
                    if current_line.is_empty() {
                        current_line = word.to_string();
                    } else if current_line.len() + 1 + word.len() <= content_width {
                        current_line.push(' ');
                        current_line.push_str(word);
                    } else {
                        let mut line_spans = vec![Span::styled("\u{2502} ".to_string(), color_style)];
                        line_spans.push(Span::raw(current_line.clone()));
                        let padding = content_width.saturating_sub(current_line.len());
                        line_spans.push(Span::raw(" ".repeat(padding)));
                        line_spans.push(Span::styled(" \u{2502}".to_string(), color_style));
                        lines.push(Line::from(line_spans));
                        current_line = word.to_string();
                    }
                }

                if !current_line.is_empty() {
                    let mut line_spans = vec![Span::styled("\u{2502} ".to_string(), color_style)];
                    line_spans.push(Span::raw(current_line.clone()));
                    let padding = content_width.saturating_sub(current_line.len());
                    line_spans.push(Span::raw(" ".repeat(padding)));
                    line_spans.push(Span::styled(" \u{2502}".to_string(), color_style));
                    lines.push(Line::from(line_spans));
                }
            }
        }
    }

    let bottom_border = format!("\u{2570}{}\u{256F}", "\u{2500}".repeat(58));
    lines.push(Line::from(Span::styled(bottom_border, color_style)));
}

/// Render a table with basic formatting
fn render_table(table: &Table, theme: &ThemeColors, lines: &mut Vec<Line<'static>>) {
    let border_style = to_ratatui_style(&theme.table_border, false);

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
        to_ratatui_style(&theme.heading, theme.heading_bold)
    } else if text_style.code {
        to_ratatui_style(&theme.code, false)
    } else {
        to_ratatui_style(&theme.body, false)
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

/// Convert theme Color to ratatui Style with RGB colors
fn to_ratatui_style(color: &lantern_core::theme::Color, bold: bool) -> Style {
    let mut style = Style::default().fg(ratatui::style::Color::Rgb(color.r, color.g, color.b));

    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    use lantern_core::slide::ListItem;
    use lantern_core::theme::Color;

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

    #[test]
    fn to_ratatui_style_converts_color() {
        let color = Color::new(255, 128, 64);
        let style = to_ratatui_style(&color, false);

        assert_eq!(style.fg, Some(ratatui::style::Color::Rgb(255, 128, 64)));
    }

    #[test]
    fn to_ratatui_style_applies_bold() {
        let color = Color::new(100, 150, 200);
        let style = to_ratatui_style(&color, true);

        assert_eq!(style.fg, Some(ratatui::style::Color::Rgb(100, 150, 200)));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn to_ratatui_style_no_bold_when_false() {
        let color = Color::new(100, 150, 200);
        let style = to_ratatui_style(&color, false);
        assert!(!style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn render_heading_uses_theme_colors() {
        let theme = ThemeColors::default();
        let blocks = vec![Block::Heading { level: 1, spans: vec![TextSpan::plain("Colored Heading")] }];
        let text = render_slide_content(&blocks, &theme);
        assert!(!text.lines.is_empty());
        assert!(!text.lines.is_empty());
    }

    #[test]
    fn apply_theme_style_respects_heading_bold() {
        let theme = ThemeColors::default();
        let text_style = TextStyle::default();
        let style = apply_theme_style(&theme, &text_style, true);
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn apply_theme_style_uses_code_color_for_code() {
        let theme = ThemeColors::default();
        let text_style = TextStyle { code: true, ..Default::default() };
        let style = apply_theme_style(&theme, &text_style, false);

        assert_eq!(
            style.fg,
            Some(ratatui::style::Color::Rgb(theme.code.r, theme.code.g, theme.code.b))
        );
    }
}
