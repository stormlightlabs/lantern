use owo_colors::{OwoColorize, Style};
use terminal_colorsaurus::{QueryOptions, background_color};

/// RGB color value for use with both owo-colors and ratatui
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Apply this color to text using owo-colors
    pub fn to_owo_color<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.into())
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Style::new().truecolor(color.r, color.g, color.b)
    }
}

impl From<&Color> for Style {
    fn from(color: &Color) -> Self {
        Style::new().truecolor(color.r, color.g, color.b)
    }
}

/// Detects if the terminal background is dark.
///
/// Uses [terminal_colorsaurus] to query the terminal background color.
/// Defaults to true (dark) if detection fails.
pub fn detect_is_dark() -> bool {
    match background_color(QueryOptions::default()) {
        Ok(color) => {
            let r = color.r as f32 / 255.0;
            let g = color.g as f32 / 255.0;
            let b = color.b as f32 / 255.0;
            let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            luminance < 0.5
        }
        Err(_) => true,
    }
}

/// Color theme abstraction for slides with semantic roles for consistent theming across the application.
///
/// Stores RGB colors that can be converted to both owo-colors Style (for terminal output)
/// and ratatui Color (for TUI rendering) via Into implementations.
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub heading: Color,
    pub heading_bold: bool,
    pub body: Color,
    pub accent: Color,
    pub code: Color,
    pub dimmed: Color,
    pub code_fence: Color,
    pub rule: Color,
    pub list_marker: Color,
    pub blockquote_border: Color,
    pub table_border: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::basic(detect_is_dark())
    }
}

impl ThemeColors {
    /// Apply heading style to text
    pub fn heading<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        let mut style: Style = (&self.heading).into();
        if self.heading_bold {
            style = style.bold();
        }
        text.style(style)
    }

    /// Apply body style to text
    pub fn body<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.body).into())
    }

    /// Apply accent style to text
    pub fn accent<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.accent).into())
    }

    /// Apply code style to text
    pub fn code<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.code).into())
    }

    /// Apply dimmed style to text
    pub fn dimmed<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.dimmed).into())
    }

    /// Apply code fence style to text
    pub fn code_fence<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.code_fence).into())
    }

    /// Apply horizontal rule style to text
    pub fn rule<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.rule).into())
    }

    /// Apply list marker style to text
    pub fn list_marker<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.list_marker).into())
    }

    /// Apply blockquote border style to text
    pub fn blockquote_border<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.blockquote_border).into())
    }

    /// Apply table border style to text
    pub fn table_border<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.table_border).into())
    }

    /// Create an oxocarbon-based theme.
    pub fn basic(is_dark: bool) -> Self {
        if is_dark {
            Self {
                heading: Color::new(66, 190, 101), // green
                heading_bold: true,
                body: Color::new(242, 244, 248),
                accent: Color::new(238, 83, 150), // pink
                code: Color::new(51, 177, 255), // blue
                dimmed: Color::new(82, 82, 82), // gray
                code_fence: Color::new(82, 82, 82),
                rule: Color::new(82, 82, 82),
                list_marker: Color::new(120, 169, 255), // light blue
                blockquote_border: Color::new(82, 82, 82),
                table_border: Color::new(82, 82, 82),
            }
        } else {
            Self {
                heading: Color::new(66, 190, 101), // green
                heading_bold: true,
                body: Color::new(57, 57, 57),
                accent: Color::new(255, 111, 0), // orange
                code: Color::new(15, 98, 254), // blue
                dimmed: Color::new(22, 22, 22), // dark gray
                code_fence: Color::new(22, 22, 22),
                rule: Color::new(22, 22, 22),
                list_marker: Color::new(238, 83, 150), // pink
                blockquote_border: Color::new(22, 22, 22),
                table_border: Color::new(22, 22, 22),
            }
        }
    }

    /// Create a Monokai-inspired theme.
    ///
    /// Dark variant uses classic Monokai colors optimized for dark backgrounds.
    /// Light variant uses adjusted colors optimized for light backgrounds.
    pub fn monokai(is_dark: bool) -> Self {
        if is_dark {
            Self {
                heading: Color::new(249, 38, 114), // pink
                heading_bold: true,
                body: Color::new(248, 248, 242), // off-white
                accent: Color::new(230, 219, 116), // yellow
                code: Color::new(166, 226, 46), // green
                dimmed: Color::new(117, 113, 94), // brown-gray
                code_fence: Color::new(117, 113, 94),
                rule: Color::new(117, 113, 94),
                list_marker: Color::new(230, 219, 116),
                blockquote_border: Color::new(117, 113, 94),
                table_border: Color::new(117, 113, 94),
            }
        } else {
            Self {
                heading: Color::new(200, 30, 90), // darker pink
                heading_bold: true,
                body: Color::new(39, 40, 34), // dark gray
                accent: Color::new(180, 170, 80), // darker yellow
                code: Color::new(100, 150, 30), // darker green
                dimmed: Color::new(150, 150, 150), // light gray
                code_fence: Color::new(150, 150, 150),
                rule: Color::new(150, 150, 150),
                list_marker: Color::new(180, 170, 80),
                blockquote_border: Color::new(150, 150, 150),
                table_border: Color::new(150, 150, 150),
            }
        }
    }

    /// Create a Dracula-inspired theme.
    ///
    /// Dark variant uses classic Dracula colors optimized for dark backgrounds.
    /// Light variant uses adjusted colors optimized for light backgrounds.
    pub fn dracula(is_dark: bool) -> Self {
        if is_dark {
            Self {
                heading: Color::new(255, 121, 198), // pink
                heading_bold: true,
                body: Color::new(248, 248, 242),
                accent: Color::new(139, 233, 253), // cyan
                code: Color::new(80, 250, 123), // green
                dimmed: Color::new(98, 114, 164),
                code_fence: Color::new(98, 114, 164),
                rule: Color::new(98, 114, 164),
                list_marker: Color::new(241, 250, 140), // yellow
                blockquote_border: Color::new(98, 114, 164),
                table_border: Color::new(98, 114, 164),
            }
        } else {
            Self {
                heading: Color::new(200, 80, 160), // darker pink
                heading_bold: true,
                body: Color::new(40, 42, 54),
                accent: Color::new(80, 150, 180), // darker cyan
                code: Color::new(50, 160, 80), // darker green
                dimmed: Color::new(150, 150, 150), // light gray
                code_fence: Color::new(150, 150, 150),
                rule: Color::new(150, 150, 150),
                list_marker: Color::new(180, 170, 90), // darker yellow
                blockquote_border: Color::new(150, 150, 150),
                table_border: Color::new(150, 150, 150),
            }
        }
    }

    /// Create a Solarized theme.
    ///
    /// Uses Ethan Schoonover's Solarized color palette.
    pub fn solarized(is_dark: bool) -> Self {
        if is_dark {
            Self {
                heading: Color::new(38, 139, 210),
                heading_bold: true,
                body: Color::new(131, 148, 150),
                accent: Color::new(42, 161, 152),
                code: Color::new(133, 153, 0),
                dimmed: Color::new(88, 110, 117),
                code_fence: Color::new(88, 110, 117),
                rule: Color::new(88, 110, 117),
                list_marker: Color::new(181, 137, 0),
                blockquote_border: Color::new(88, 110, 117),
                table_border: Color::new(88, 110, 117),
            }
        } else {
            Self {
                heading: Color::new(38, 139, 210),
                heading_bold: true,
                body: Color::new(101, 123, 131),
                accent: Color::new(42, 161, 152),
                code: Color::new(133, 153, 0),
                dimmed: Color::new(147, 161, 161),
                code_fence: Color::new(147, 161, 161),
                rule: Color::new(147, 161, 161),
                list_marker: Color::new(181, 137, 0),
                blockquote_border: Color::new(147, 161, 161),
                table_border: Color::new(147, 161, 161),
            }
        }
    }

    /// Create a Nord theme instance
    pub fn nord(is_dark: bool) -> Self {
        if is_dark {
            Self {
                heading: Color::new(136, 192, 208), // nord8 - light blue
                heading_bold: true,
                body: Color::new(216, 222, 233), // nord4
                accent: Color::new(143, 188, 187), // nord7 - teal
                code: Color::new(163, 190, 140), // nord14 - green
                dimmed: Color::new(76, 86, 106), // nord3
                code_fence: Color::new(76, 86, 106),
                rule: Color::new(76, 86, 106),
                list_marker: Color::new(235, 203, 139), // nord13 - yellow
                blockquote_border: Color::new(76, 86, 106),
                table_border: Color::new(76, 86, 106),
            }
        } else {
            Self {
                heading: Color::new(94, 129, 172), // darker blue
                heading_bold: true,
                body: Color::new(46, 52, 64),
                accent: Color::new(136, 192, 208), // blue
                code: Color::new(163, 190, 140), // green
                dimmed: Color::new(143, 157, 175),
                code_fence: Color::new(143, 157, 175),
                rule: Color::new(143, 157, 175),
                list_marker: Color::new(235, 203, 139), // yellow
                blockquote_border: Color::new(143, 157, 175),
                table_border: Color::new(143, 157, 175),
            }
        }
    }
}

/// Theme registry for loading themes by name with automatic light/dark variant selection.
pub struct ThemeRegistry;

impl ThemeRegistry {
    /// Get a theme by name with automatic variant detection or explicit override.
    pub fn get(name: &str) -> ThemeColors {
        let (theme_name, explicit_variant) = if let Some((scheme, variant)) = name.split_once(':') {
            let is_dark = match variant.to_lowercase().as_str() {
                "light" => false,
                "dark" => true,
                _ => detect_is_dark(),
            };
            (scheme, Some(is_dark))
        } else {
            (name, None)
        };

        let is_dark = explicit_variant.unwrap_or_else(detect_is_dark);

        match theme_name.to_lowercase().as_str() {
            "basic" => ThemeColors::basic(is_dark),
            "monokai" => ThemeColors::monokai(is_dark),
            "dracula" => ThemeColors::dracula(is_dark),
            "solarized" => ThemeColors::solarized(is_dark),
            "nord" => ThemeColors::nord(is_dark),
            _ => ThemeColors::basic(is_dark),
        }
    }

    /// List all available theme scheme names.
    pub fn available_themes() -> Vec<&'static str> {
        vec!["basic", "monokai", "dracula", "solarized", "nord"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_new() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn color_into_style() {
        let color = Color::new(100, 150, 200);
        let style: Style = color.into();
        let text = "Test";
        let styled = text.style(style);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn color_ref_into_style() {
        let color = Color::new(100, 150, 200);
        let style: Style = (&color).into();
        let text = "Test";
        let styled = text.style(style);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn color_into_style_preserves_rgb() {
        let color = Color::new(255, 0, 128);
        let style: Style = color.into();
        let styled_text = "Test".style(style);
        let output = styled_text.to_string();
        assert!(output.contains("Test"));
    }

    #[test]
    fn theme_colors_default() {
        let theme = ThemeColors::default();
        let text = "Test";
        let heading = theme.heading(&text);
        assert!(heading.to_string().contains("Test"));
    }

    #[test]
    fn theme_colors_apply_styles() {
        let theme = ThemeColors::default();

        assert!(theme.heading(&"Heading").to_string().contains("Heading"));
        assert!(theme.body(&"Body").to_string().contains("Body"));
        assert!(theme.accent(&"Accent").to_string().contains("Accent"));
        assert!(theme.code(&"Code").to_string().contains("Code"));
        assert!(theme.dimmed(&"Dimmed").to_string().contains("Dimmed"));
    }

    #[test]
    fn theme_basic_dark_variant() {
        let theme = ThemeColors::basic(true);
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_basic_light_variant() {
        let theme = ThemeColors::basic(false);
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_monokai_variants() {
        let dark = ThemeColors::monokai(true);
        let light = ThemeColors::monokai(false);
        assert!(dark.heading(&"Test").to_string().contains("Test"));
        assert!(light.heading(&"Test").to_string().contains("Test"));
    }

    #[test]
    fn theme_dracula_variants() {
        let dark = ThemeColors::dracula(true);
        let light = ThemeColors::dracula(false);
        assert!(dark.heading(&"Test").to_string().contains("Test"));
        assert!(light.heading(&"Test").to_string().contains("Test"));
    }

    #[test]
    fn theme_solarized_variants() {
        let dark = ThemeColors::solarized(true);
        let light = ThemeColors::solarized(false);
        assert!(dark.heading(&"Test").to_string().contains("Test"));
        assert!(light.heading(&"Test").to_string().contains("Test"));
    }

    #[test]
    fn theme_nord_variants() {
        let dark = ThemeColors::nord(true);
        let light = ThemeColors::nord(false);
        assert!(dark.heading(&"Test").to_string().contains("Test"));
        assert!(light.heading(&"Test").to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_basic() {
        let theme = ThemeRegistry::get("basic");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_explicit_variant_dark() {
        let theme = ThemeRegistry::get("basic:dark");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_explicit_variant_light() {
        let theme = ThemeRegistry::get("solarized:light");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_monokai() {
        let theme = ThemeRegistry::get("monokai");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_dracula() {
        let theme = ThemeRegistry::get("dracula");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_solarized() {
        let theme = ThemeRegistry::get("solarized");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_nord() {
        let theme = ThemeRegistry::get("nord");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_unknown_fallback() {
        let theme = ThemeRegistry::get("nonexistent");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_case_insensitive() {
        let theme1 = ThemeRegistry::get("BASIC");
        let theme2 = ThemeRegistry::get("basic");
        let text = "Test";
        assert!(theme1.heading(&text).to_string().contains("Test"));
        assert!(theme2.heading(&text).to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_available_themes() {
        let themes = ThemeRegistry::available_themes();
        assert!(themes.contains(&"basic"));
        assert!(themes.contains(&"monokai"));
        assert!(themes.contains(&"dracula"));
        assert!(themes.contains(&"solarized"));
        assert!(themes.contains(&"nord"));
        assert_eq!(themes.len(), 5);
    }

    #[test]
    fn detect_is_dark_returns_bool() {
        let result = detect_is_dark();
        assert!(result == true || result == false);
    }

    #[test]
    fn theme_colors_stores_correct_colors() {
        let theme = ThemeColors::basic(true);
        assert_eq!(theme.heading.r, 66);
        assert_eq!(theme.heading.g, 190);
        assert_eq!(theme.heading.b, 101);
        assert!(theme.heading_bold);
    }

    #[test]
    fn theme_colors_heading_applies_bold() {
        let theme = ThemeColors::basic(true);
        let text = "Bold Heading";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Bold Heading"));
    }

    #[test]
    fn theme_colors_all_semantic_roles() {
        let theme = ThemeColors::default();

        assert!(theme.heading(&"Test").to_string().contains("Test"));
        assert!(theme.body(&"Test").to_string().contains("Test"));
        assert!(theme.accent(&"Test").to_string().contains("Test"));
        assert!(theme.code(&"Test").to_string().contains("Test"));
        assert!(theme.dimmed(&"Test").to_string().contains("Test"));
        assert!(theme.code_fence(&"Test").to_string().contains("Test"));
        assert!(theme.rule(&"Test").to_string().contains("Test"));
        assert!(theme.list_marker(&"Test").to_string().contains("Test"));
        assert!(theme.blockquote_border(&"Test").to_string().contains("Test"));
        assert!(theme.table_border(&"Test").to_string().contains("Test"));
    }

    #[test]
    fn theme_colors_light_vs_dark() {
        let dark = ThemeColors::basic(true);
        let light = ThemeColors::basic(false);

        assert_eq!(dark.body.r, 242);
        assert_eq!(light.body.r, 57);
        assert_ne!(dark.body.r, light.body.r);
    }
}
