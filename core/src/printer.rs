use crate::highlighter;
use crate::slide::{Block, CodeBlock, List, Table, TextSpan, TextStyle};
use crate::theme::ThemeColors;

/// Print slides to stdout with formatted output
///
/// Renders slides as plain text with ANSI colors and width constraints.
pub fn print_slides_to_stdout(
    slides: &[crate::slide::Slide], theme: &ThemeColors, width: usize,
) -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    print_slides(&mut handle, slides, theme, width)
}

/// Print slides to any writer with formatted output
pub fn print_slides<W: std::io::Write>(
    writer: &mut W, slides: &[crate::slide::Slide], theme: &ThemeColors, width: usize,
) -> std::io::Result<()> {
    for (idx, slide) in slides.iter().enumerate() {
        if idx > 0 {
            writeln!(writer)?;
            let sep_text = "═".repeat(width);
            let separator = theme.rule(&sep_text);
            writeln!(writer, "{separator}")?;
            writeln!(writer)?;
        }

        print_slide(writer, slide, theme, width)?;
    }

    Ok(())
}

/// Print a single slide with formatted blocks
fn print_slide<W: std::io::Write>(
    writer: &mut W, slide: &crate::slide::Slide, theme: &ThemeColors, width: usize,
) -> std::io::Result<()> {
    for block in &slide.blocks {
        print_block(writer, block, theme, width, 0)?;
        writeln!(writer)?;
    }

    Ok(())
}

/// Print a single block with appropriate formatting
fn print_block<W: std::io::Write>(
    writer: &mut W, block: &Block, theme: &ThemeColors, width: usize, indent: usize,
) -> std::io::Result<()> {
    match block {
        Block::Heading { level, spans } => {
            print_heading(writer, *level, spans, theme)?;
        }
        Block::Paragraph { spans } => {
            print_paragraph(writer, spans, theme, width, indent)?;
        }
        Block::Code(code) => {
            print_code_block(writer, code, theme, width)?;
        }
        Block::List(list) => {
            print_list(writer, list, theme, width, indent)?;
        }
        Block::Rule => {
            let rule_text = "─".repeat(width.saturating_sub(indent));
            let rule = theme.rule(&rule_text);
            writeln!(writer, "{}{}", " ".repeat(indent), rule)?;
        }
        Block::BlockQuote { blocks } => {
            print_blockquote(writer, blocks, theme, width, indent)?;
        }
        Block::Table(table) => {
            print_table(writer, table, theme, width)?;
        }
    }

    Ok(())
}

/// Print a heading with level-appropriate styling using Unicode block symbols
fn print_heading<W: std::io::Write>(
    writer: &mut W, level: u8, spans: &[TextSpan], theme: &ThemeColors,
) -> std::io::Result<()> {
    let prefix = match level {
        1 => "▉ ",
        2 => "▓ ",
        3 => "▒ ",
        4 => "░ ",
        _ => "▌ ",
    };

    write!(writer, "{}", theme.heading(&prefix))?;

    for span in spans {
        print_span(writer, span, theme, true)?;
    }

    writeln!(writer)?;
    Ok(())
}

/// Print a paragraph with word wrapping
fn print_paragraph<W: std::io::Write>(
    writer: &mut W, spans: &[TextSpan], theme: &ThemeColors, width: usize, indent: usize,
) -> std::io::Result<()> {
    let indent_str = " ".repeat(indent);
    let effective_width = width.saturating_sub(indent);

    let text = spans.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join("");

    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= effective_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            write!(writer, "{indent_str}")?;
            for span in spans {
                if current_line.contains(&span.text) {
                    print_span(writer, span, theme, false)?;
                    break;
                }
            }
            if !spans.is_empty() && !current_line.is_empty() {
                write!(writer, "{}", theme.body(&current_line))?;
            }
            writeln!(writer)?;
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        write!(writer, "{indent_str}")?;
        for span in spans {
            print_span(writer, span, theme, false)?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

/// Print a code block with syntax highlighting
fn print_code_block<W: std::io::Write>(
    writer: &mut W, code: &CodeBlock, theme: &ThemeColors, width: usize,
) -> std::io::Result<()> {
    if let Some(lang) = &code.language {
        writeln!(writer, "{}", theme.code_fence(&format!("```{lang}")))?;
    } else {
        writeln!(writer, "{}", theme.code_fence(&"```"))?;
    }

    let highlighted_lines = highlighter::highlight_code(&code.code, code.language.as_deref(), theme);

    for tokens in highlighted_lines {
        let mut line_length = 0;
        for token in tokens {
            if line_length + token.text.len() > width - 4 {
                let remaining = (width - 4).saturating_sub(line_length);
                if remaining > 0 {
                    let trimmed = &token.text[..remaining.min(token.text.len())];
                    write!(writer, "{}", token.color.to_owo_color(&trimmed))?;
                }
                break;
            }
            write!(writer, "{}", token.color.to_owo_color(&token.text))?;
            line_length += token.text.len();
        }
        writeln!(writer)?;
    }

    writeln!(writer, "{}", theme.code_fence(&"```"))?;
    Ok(())
}

/// Print a list with bullets or numbers
fn print_list<W: std::io::Write>(
    writer: &mut W, list: &List, theme: &ThemeColors, _width: usize, indent: usize,
) -> std::io::Result<()> {
    for (idx, item) in list.items.iter().enumerate() {
        let marker = if list.ordered { format!("{}. ", idx + 1) } else { "• ".to_string() };

        write!(writer, "{}", " ".repeat(indent))?;
        write!(writer, "{}", theme.list_marker(&marker))?;

        for span in &item.spans {
            print_span(writer, span, theme, false)?;
        }

        writeln!(writer)?;

        if let Some(nested) = &item.nested {
            print_list(writer, nested, theme, _width, indent + 2)?;
        }
    }

    Ok(())
}

/// Print a blockquote with border
fn print_blockquote<W: std::io::Write>(
    writer: &mut W, blocks: &[Block], theme: &ThemeColors, width: usize, indent: usize,
) -> std::io::Result<()> {
    for block in blocks {
        match block {
            Block::Paragraph { spans } => {
                write!(writer, "{}", " ".repeat(indent))?;
                write!(writer, "{}", theme.blockquote_border(&"│ "))?;
                for span in spans {
                    print_span(writer, span, theme, false)?;
                }
                writeln!(writer)?;
            }
            _ => {
                write!(writer, "{}", " ".repeat(indent))?;
                write!(writer, "{}", theme.blockquote_border(&"│ "))?;
                print_block(writer, block, theme, width, indent + 2)?;
            }
        }
    }

    Ok(())
}

/// Print a table with borders and proper column width calculation
///
/// Calculates column widths based on content and distributes available space
fn print_table<W: std::io::Write>(
    writer: &mut W, table: &Table, theme: &ThemeColors, width: usize,
) -> std::io::Result<()> {
    let col_count = table.headers.len();
    if col_count == 0 {
        return Ok(());
    }

    let col_widths = calculate_column_widths(table, width);

    if !table.headers.is_empty() {
        print_table_row(writer, &table.headers, &col_widths, theme, true)?;

        let separator = build_table_separator(&col_widths);
        writeln!(writer, "{}", theme.table_border(&separator))?;
    }

    for row in &table.rows {
        print_table_row(writer, row, &col_widths, theme, false)?;
    }

    Ok(())
}

/// Calculate column widths based on content and available space
fn calculate_column_widths(table: &Table, max_width: usize) -> Vec<usize> {
    let col_count = table.headers.len();
    if col_count == 0 {
        return vec![];
    }

    let mut col_widths = vec![0; col_count];

    for (col_idx, header) in table.headers.iter().enumerate() {
        let content_len: usize = header.iter().map(|s| s.text.len()).sum();
        col_widths[col_idx] = content_len.max(3);
    }

    for row in &table.rows {
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx < col_widths.len() {
                let content_len = cell.iter().map(|s| s.text.len()).sum();
                col_widths[col_idx] = col_widths[col_idx].max(content_len);
            }
        }
    }

    let separator_width = (col_count - 1) * 3;
    let padding_width = col_count * 2;
    let available_width = max_width.saturating_sub(separator_width + padding_width);

    let total_content_width: usize = col_widths.iter().sum();

    if total_content_width > available_width {
        let scale_factor = available_width as f64 / total_content_width as f64;
        for width in &mut col_widths {
            *width = ((*width as f64 * scale_factor).ceil() as usize).max(3);
        }
    }

    col_widths
}

/// Build a table separator line with proper column separators
fn build_table_separator(col_widths: &[usize]) -> String {
    let mut separator = String::new();
    for (idx, &width) in col_widths.iter().enumerate() {
        if idx > 0 {
            separator.push_str("─┼─");
        }
        separator.push_str(&"─".repeat(width + 2));
    }
    separator
}

/// Print a single table row with proper padding and alignment
fn print_table_row<W: std::io::Write>(
    writer: &mut W, cells: &[Vec<TextSpan>], col_widths: &[usize], theme: &ThemeColors, is_header: bool,
) -> std::io::Result<()> {
    for (idx, cell) in cells.iter().enumerate() {
        if idx > 0 {
            write!(writer, "{}", theme.table_border(&" │ "))?;
        } else {
            write!(writer, " ")?;
        }

        let col_width = col_widths.get(idx).copied().unwrap_or(10);
        let content: String = cell.iter().map(|s| s.text.as_str()).collect();
        let content_len = content.len();

        for span in cell {
            print_span(writer, span, theme, is_header)?;
        }

        if content_len < col_width {
            write!(writer, "{}", " ".repeat(col_width - content_len))?;
        }

        write!(writer, " ")?;
    }
    writeln!(writer)?;

    Ok(())
}

/// Print a text span with styling
fn print_span<W: std::io::Write>(
    writer: &mut W, span: &TextSpan, theme: &ThemeColors, is_heading: bool,
) -> std::io::Result<()> {
    let text = &span.text;
    let style = &span.style;

    if is_heading {
        write!(writer, "{}", apply_text_style(&theme.heading(text), style))?;
    } else if style.code {
        write!(writer, "{}", apply_text_style(&theme.code(text), style))?;
    } else {
        write!(writer, "{}", apply_text_style(&theme.body(text), style))?;
    }

    Ok(())
}

/// Apply text style modifiers to styled text
fn apply_text_style<T: std::fmt::Display>(styled: &owo_colors::Styled<T>, text_style: &TextStyle) -> String {
    let mut result = styled.to_string();

    if text_style.bold {
        result = format!("\x1b[1m{result}\x1b[22m");
    }
    if text_style.italic {
        result = format!("\x1b[3m{result}\x1b[23m");
    }
    if text_style.strikethrough {
        result = format!("\x1b[9m{result}\x1b[29m");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slide::Slide;
    use crate::slide::{Alignment, Table};

    #[test]
    fn print_empty_slides() {
        let slides: Vec<Slide> = vec![];
        let theme = ThemeColors::default();
        let mut output = Vec::new();

        let result = print_slides(&mut output, &slides, &theme, 80);
        assert!(result.is_ok());
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn print_single_heading() {
        let slide = Slide::with_blocks(vec![Block::Heading {
            level: 1,
            spans: vec![TextSpan::plain("Hello World")],
        }]);
        let theme = ThemeColors::default();
        let mut output = Vec::new();

        let result = print_slides(&mut output, &[slide], &theme, 80);
        assert!(result.is_ok());
        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("Hello World"));
    }

    #[test]
    fn print_paragraph_with_wrapping() {
        let long_text = "This is a very long paragraph that should wrap when printed to stdout with a width constraint applied to ensure readability.";
        let slide = Slide::with_blocks(vec![Block::Paragraph { spans: vec![TextSpan::plain(long_text)] }]);
        let theme = ThemeColors::default();
        let mut output = Vec::new();

        let result = print_slides(&mut output, &[slide], &theme, 40);
        assert!(result.is_ok());
    }

    #[test]
    fn print_code_block() {
        let slide = Slide::with_blocks(vec![Block::Code(CodeBlock::with_language(
            "rust",
            "fn main() {\n    println!(\"Hello\");\n}",
        ))]);
        let theme = ThemeColors::default();
        let mut output = Vec::new();

        let result = print_slides(&mut output, &[slide], &theme, 80);
        assert!(result.is_ok());

        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("```rust"));
        assert!(text.contains("fn") && text.contains("main"));
        assert!(text.contains("println"));
    }

    #[test]
    fn print_multiple_slides() {
        let slides = vec![
            Slide::with_blocks(vec![Block::Heading {
                level: 1,
                spans: vec![TextSpan::plain("Slide 1")],
            }]),
            Slide::with_blocks(vec![Block::Heading {
                level: 1,
                spans: vec![TextSpan::plain("Slide 2")],
            }]),
        ];

        let theme = ThemeColors::default();
        let mut output = Vec::new();
        let result = print_slides(&mut output, &slides, &theme, 80);
        assert!(result.is_ok());

        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("Slide 1"));
        assert!(text.contains("Slide 2"));
    }

    #[test]
    fn print_table_with_headers() {
        let table = Table {
            headers: vec![
                vec![TextSpan::plain("Name")],
                vec![TextSpan::plain("Age")],
                vec![TextSpan::plain("City")],
            ],
            rows: vec![
                vec![
                    vec![TextSpan::plain("Alice")],
                    vec![TextSpan::plain("30")],
                    vec![TextSpan::plain("NYC")],
                ],
                vec![
                    vec![TextSpan::plain("Bob")],
                    vec![TextSpan::plain("25")],
                    vec![TextSpan::plain("LA")],
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Left, Alignment::Left],
        };

        let slide = Slide::with_blocks(vec![Block::Table(table)]);
        let theme = ThemeColors::default();
        let mut output = Vec::new();

        let result = print_slides(&mut output, &[slide], &theme, 80);
        assert!(result.is_ok());

        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("Name"));
        assert!(text.contains("Age"));
        assert!(text.contains("City"));
        assert!(text.contains("Alice"));
        assert!(text.contains("Bob"));
        assert!(text.contains("│"));
        assert!(text.contains("─"));
    }

    #[test]
    fn print_table_with_column_width_calculation() {
        let table = Table {
            headers: vec![vec![TextSpan::plain("Short")], vec![TextSpan::plain("Long Header")]],
            rows: vec![
                vec![vec![TextSpan::plain("A")], vec![TextSpan::plain("B")]],
                vec![vec![TextSpan::plain("Very Long Content")], vec![TextSpan::plain("X")]],
            ],
            alignments: vec![Alignment::Left, Alignment::Left],
        };

        let col_widths = calculate_column_widths(&table, 80);

        assert_eq!(col_widths.len(), 2);
        assert!(col_widths[0] >= 17);
        assert!(col_widths[1] >= 11);
    }

    #[test]
    fn print_table_empty_headers() {
        let table = Table { headers: vec![], rows: vec![], alignments: vec![] };

        let slide = Slide::with_blocks(vec![Block::Table(table)]);
        let theme = ThemeColors::default();
        let mut output = Vec::new();

        let result = print_slides(&mut output, &[slide], &theme, 80);
        assert!(result.is_ok());
    }

    #[test]
    fn calculate_column_widths_scales_to_fit() {
        let table = Table {
            headers: vec![
                vec![TextSpan::plain("A".repeat(50))],
                vec![TextSpan::plain("B".repeat(50))],
            ],
            rows: vec![],
            alignments: vec![Alignment::Left, Alignment::Left],
        };

        let col_widths = calculate_column_widths(&table, 40);
        let total_width: usize = col_widths.iter().sum();

        assert!(total_width <= 40);
    }

    #[test]
    fn build_table_separator_correct_format() {
        let col_widths = vec![5, 10, 7];
        let separator = build_table_separator(&col_widths);

        assert!(separator.contains("─┼─"));
        assert!(separator.contains("─"));
    }
}
