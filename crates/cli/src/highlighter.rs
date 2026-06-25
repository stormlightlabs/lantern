use std::sync::OnceLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::theme::{Color, ThemeColors};

/// Global syntax set (lazy-initialized)
static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();

/// Global theme set (lazy-initialized)
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

/// Get the global syntax set
pub fn syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

/// Get the global theme set
pub fn theme_set() -> &'static ThemeSet {
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

/// A highlighted token with text and color
#[derive(Debug, Clone)]
pub struct HighlightedToken {
    pub text: String,
    pub color: Color,
}

/// Highlight code using syntect and map to theme colors
///
/// Returns a vector of lines, where each line is a vector of highlighted tokens.
/// If the language is not recognized or highlighting fails, returns the code with default styling.
pub fn highlight_code(code: &str, language: Option<&str>, theme_colors: &ThemeColors) -> Vec<Vec<HighlightedToken>> {
    let ss = syntax_set();

    let syntax = language
        .and_then(|lang| ss.find_syntax_by_token(lang))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let syntect_theme = get_syntect_theme(theme_colors);

    let mut highlighter = HighlightLines::new(syntax, syntect_theme);
    let mut result = Vec::new();

    for line in LinesWithEndings::from(code) {
        let Ok(ranges) = highlighter.highlight_line(line, ss) else {
            result.push(vec![HighlightedToken {
                text: line.to_string(),
                color: theme_colors.code,
            }]);
            continue;
        };

        let mut tokens = Vec::new();
        for (style, text) in ranges {
            let color = Color::from_syntect(style.foreground);
            tokens.push(HighlightedToken { text: text.to_string(), color });
        }
        result.push(tokens);
    }

    result
}

/// Get the appropriate syntect theme based on the current theme
fn get_syntect_theme(theme_colors: &ThemeColors) -> &'static Theme {
    let ts = theme_set();
    let is_dark = is_dark_theme(theme_colors);

    if is_dark {
        ts.themes
            .get("base16-ocean.dark")
            .or_else(|| ts.themes.get("Solarized (dark)"))
            .or_else(|| ts.themes.get("base16-mocha.dark"))
            .unwrap_or_else(|| ts.themes.values().next().unwrap())
    } else {
        ts.themes
            .get("base16-ocean.light")
            .or_else(|| ts.themes.get("Solarized (light)"))
            .or_else(|| ts.themes.get("InspiredGitHub"))
            .unwrap_or_else(|| ts.themes.values().next().unwrap())
    }
}

/// Detect if a theme is dark based on its colors
fn is_dark_theme(theme_colors: &ThemeColors) -> bool {
    let body = theme_colors.body;
    let luminance = 0.299 * body.r as f32 + 0.587 * body.g as f32 + 0.114 * body.b as f32;
    luminance > 128.0
}

impl Color {
    /// Create a Color from syntect's RGB color
    pub fn from_syntect(color: syntect::highlighting::Color) -> Self {
        Self { r: color.r, g: color.g, b: color.b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_set_loads_successfully() {
        let ss = syntax_set();
        assert!(!ss.syntaxes().is_empty());
    }

    #[test]
    fn theme_set_loads_successfully() {
        let ts = theme_set();
        assert!(!ts.themes.is_empty());
    }

    #[test]
    fn highlight_code_with_rust_syntax() {
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let theme = ThemeColors::default();
        let result = highlight_code(code, Some("rust"), &theme);

        assert_eq!(result.len(), 3);
        assert!(!result[0].is_empty());
        assert!(!result[1].is_empty());
        assert!(!result[2].is_empty());
    }

    #[test]
    fn highlight_code_with_unknown_language() {
        let code = "some random text";
        let theme = ThemeColors::default();
        let result = highlight_code(code, Some("unknown-lang-xyz"), &theme);
        assert_eq!(result.len(), 1);
        assert!(!result[0].is_empty());
    }

    #[test]
    fn highlight_code_without_language() {
        let code = "plain text\nno highlighting";
        let theme = ThemeColors::default();
        let result = highlight_code(code, None, &theme);
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_empty());
        assert!(!result[1].is_empty());
    }

    #[test]
    fn highlight_code_empty_string() {
        let theme = ThemeColors::default();
        let result = highlight_code("", Some("rust"), &theme);
        assert!(result.is_empty() || (result.len() == 1 && result[0].is_empty()));
    }

    #[test]
    fn highlight_code_with_python_syntax() {
        let code = "def hello():\n    print(\"world\")";
        let theme = ThemeColors::default();
        let result = highlight_code(code, Some("python"), &theme);

        assert_eq!(result.len(), 2);
        assert!(!result[0].is_empty());
        assert!(!result[1].is_empty());
    }

    #[test]
    fn highlight_code_preserves_line_count() {
        let code = "line1\nline2\nline3\nline4";
        let theme = ThemeColors::default();
        let result = highlight_code(code, Some("rust"), &theme);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn color_from_syntect_conversion() {
        let syntect_color = syntect::highlighting::Color { r: 255, g: 128, b: 64, a: 255 };
        let color = Color::from_syntect(syntect_color);

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn is_dark_theme_detects_dark() {
        let dark_theme = ThemeColors {
            heading: Color::new(200, 200, 200),
            heading_bold: true,
            body: Color::new(180, 180, 180),
            accent: Color::new(100, 150, 200),
            code: Color::new(150, 150, 150),
            dimmed: Color::new(100, 100, 100),
            code_fence: Color::new(120, 120, 120),
            rule: Color::new(100, 100, 100),
            list_marker: Color::new(150, 150, 150),
            blockquote_border: Color::new(120, 120, 120),
            table_border: Color::new(120, 120, 120),
            emphasis: Color::new(160, 160, 160),
            strong: Color::new(190, 190, 190),
            link: Color::new(140, 180, 220),
            inline_code_bg: Color::new(50, 50, 50),
            ui_border: Color::new(80, 80, 80),
            ui_title: Color::new(200, 200, 200),
            ui_text: Color::new(220, 220, 220),
            ui_background: Color::new(30, 30, 30),
            admonition_note: Color::new(100, 150, 200),
            admonition_tip: Color::new(150, 100, 200),
            admonition_warning: Color::new(200, 150, 50),
            admonition_danger: Color::new(200, 50, 50),
            admonition_success: Color::new(50, 200, 100),
            admonition_info: Color::new(100, 200, 200),
        };

        assert!(is_dark_theme(&dark_theme));
    }

    #[test]
    fn is_dark_theme_detects_light() {
        let light_theme = ThemeColors {
            heading: Color::new(50, 50, 50),
            heading_bold: true,
            body: Color::new(30, 30, 30),
            accent: Color::new(0, 100, 200),
            code: Color::new(60, 60, 60),
            dimmed: Color::new(100, 100, 100),
            code_fence: Color::new(80, 80, 80),
            rule: Color::new(100, 100, 100),
            list_marker: Color::new(50, 50, 50),
            blockquote_border: Color::new(80, 80, 80),
            table_border: Color::new(80, 80, 80),
            emphasis: Color::new(70, 70, 70),
            strong: Color::new(40, 40, 40),
            link: Color::new(0, 80, 160),
            inline_code_bg: Color::new(240, 240, 240),
            ui_border: Color::new(180, 180, 180),
            ui_title: Color::new(40, 40, 40),
            ui_text: Color::new(20, 20, 20),
            ui_background: Color::new(250, 250, 250),
            admonition_note: Color::new(0, 100, 200),
            admonition_tip: Color::new(100, 0, 200),
            admonition_warning: Color::new(200, 100, 0),
            admonition_danger: Color::new(200, 0, 0),
            admonition_success: Color::new(0, 150, 50),
            admonition_info: Color::new(0, 150, 200),
        };

        assert!(!is_dark_theme(&light_theme));
    }

    #[test]
    fn get_syntect_theme_returns_valid_theme() {
        let theme = ThemeColors::default();
        let syntect_theme = get_syntect_theme(&theme);
        assert!(syntect_theme.settings.background.is_some() || syntect_theme.settings.foreground.is_some());
    }

    #[test]
    fn highlight_code_handles_multiline_strings() {
        let code = r#"let s = "hello
world";"#;
        let theme = ThemeColors::default();
        let result = highlight_code(code, Some("rust"), &theme);
        assert_eq!(result.len(), 2);
    }
}
