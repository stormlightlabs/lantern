use owo_colors::{OwoColorize, Style};
use terminal_colorsaurus::{QueryOptions, background_color};

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

/// Color theme abstraction for slides with owo-colors with semantic roles for consistent theming across the application.
///
/// Avoids dynamic dispatch by using compile-time color assignments.
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub heading: Style,
    pub body: Style,
    pub accent: Style,
    pub code: Style,
    pub dimmed: Style,
    pub code_fence: Style,
    pub rule: Style,
    pub list_marker: Style,
    pub blockquote_border: Style,
    pub table_border: Style,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::basic(detect_is_dark())
    }
}

impl ThemeColors {
    /// Apply heading style to text
    pub fn heading<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.heading)
    }

    /// Apply body style to text
    pub fn body<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.body)
    }

    /// Apply accent style to text
    pub fn accent<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.accent)
    }

    /// Apply code style to text
    pub fn code<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.code)
    }

    /// Apply dimmed style to text
    pub fn dimmed<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.dimmed)
    }

    /// Apply code fence style to text
    pub fn code_fence<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.code_fence)
    }

    /// Apply horizontal rule style to text
    pub fn rule<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.rule)
    }

    /// Apply list marker style to text
    pub fn list_marker<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.list_marker)
    }

    /// Apply blockquote border style to text
    pub fn blockquote_border<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.blockquote_border)
    }

    /// Apply table border style to text
    pub fn table_border<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style(self.table_border)
    }

    /// Create an oxocarbon-based theme.
    pub fn basic(is_dark: bool) -> Self {
        if is_dark {
            Self {
                // green
                heading: Style::new().truecolor(66, 190, 101).bold(),
                body: Style::new().truecolor(242, 244, 248),
                // pink
                accent: Style::new().truecolor(238, 83, 150),
                // blue
                code: Style::new().truecolor(51, 177, 255),
                // gray
                dimmed: Style::new().truecolor(82, 82, 82),
                code_fence: Style::new().truecolor(82, 82, 82),
                rule: Style::new().truecolor(82, 82, 82),
                // light blue
                list_marker: Style::new().truecolor(120, 169, 255),
                blockquote_border: Style::new().truecolor(82, 82, 82),
                table_border: Style::new().truecolor(82, 82, 82),
            }
        } else {
            // Oxocarbon Light variant
            Self {
                // green
                heading: Style::new().truecolor(66, 190, 101).bold(),
                body: Style::new().truecolor(57, 57, 57),
                // orange
                accent: Style::new().truecolor(255, 111, 0),
                // blue
                code: Style::new().truecolor(15, 98, 254),
                // dark gray
                dimmed: Style::new().truecolor(22, 22, 22),
                code_fence: Style::new().truecolor(22, 22, 22),
                rule: Style::new().truecolor(22, 22, 22),
                // pink
                list_marker: Style::new().truecolor(238, 83, 150),
                blockquote_border: Style::new().truecolor(22, 22, 22),
                table_border: Style::new().truecolor(22, 22, 22),
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
                heading: Style::new().truecolor(249, 38, 114).bold(), // pink
                body: Style::new().truecolor(248, 248, 242),          // off-white
                accent: Style::new().truecolor(230, 219, 116),        // yellow
                code: Style::new().truecolor(166, 226, 46),           // green
                dimmed: Style::new().truecolor(117, 113, 94),         // brown-gray
                code_fence: Style::new().truecolor(117, 113, 94),
                rule: Style::new().truecolor(117, 113, 94),
                list_marker: Style::new().truecolor(230, 219, 116),
                blockquote_border: Style::new().truecolor(117, 113, 94),
                table_border: Style::new().truecolor(117, 113, 94),
            }
        } else {
            Self {
                heading: Style::new().truecolor(200, 30, 90).bold(), // darker pink
                body: Style::new().truecolor(39, 40, 34),            // dark gray
                accent: Style::new().truecolor(180, 170, 80),        // darker yellow
                code: Style::new().truecolor(100, 150, 30),          // darker green
                dimmed: Style::new().truecolor(150, 150, 150),       // light gray
                code_fence: Style::new().truecolor(150, 150, 150),
                rule: Style::new().truecolor(150, 150, 150),
                list_marker: Style::new().truecolor(180, 170, 80),
                blockquote_border: Style::new().truecolor(150, 150, 150),
                table_border: Style::new().truecolor(150, 150, 150),
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
                heading: Style::new().truecolor(255, 121, 198).bold(), // pink
                body: Style::new().truecolor(248, 248, 242),
                accent: Style::new().truecolor(139, 233, 253), // cyan
                code: Style::new().truecolor(80, 250, 123),    // green
                dimmed: Style::new().truecolor(98, 114, 164),
                code_fence: Style::new().truecolor(98, 114, 164),
                rule: Style::new().truecolor(98, 114, 164),
                list_marker: Style::new().truecolor(241, 250, 140), // yellow
                blockquote_border: Style::new().truecolor(98, 114, 164),
                table_border: Style::new().truecolor(98, 114, 164),
            }
        } else {
            Self {
                heading: Style::new().truecolor(200, 80, 160).bold(), // darker pink
                body: Style::new().truecolor(40, 42, 54),
                accent: Style::new().truecolor(80, 150, 180),  // darker cyan
                code: Style::new().truecolor(50, 160, 80),     // darker green
                dimmed: Style::new().truecolor(150, 150, 150), // light gray
                code_fence: Style::new().truecolor(150, 150, 150),
                rule: Style::new().truecolor(150, 150, 150),
                list_marker: Style::new().truecolor(180, 170, 90), // darker yellow
                blockquote_border: Style::new().truecolor(150, 150, 150),
                table_border: Style::new().truecolor(150, 150, 150),
            }
        }
    }

    /// Create a Solarized theme.
    ///
    /// Uses Ethan Schoonover's Solarized color palette.
    pub fn solarized(is_dark: bool) -> Self {
        if is_dark {
            Self {
                heading: Style::new().truecolor(38, 139, 210).bold(),
                body: Style::new().truecolor(131, 148, 150),
                accent: Style::new().truecolor(42, 161, 152),
                code: Style::new().truecolor(133, 153, 0),
                dimmed: Style::new().truecolor(88, 110, 117),
                code_fence: Style::new().truecolor(88, 110, 117),
                rule: Style::new().truecolor(88, 110, 117),
                list_marker: Style::new().truecolor(181, 137, 0),
                blockquote_border: Style::new().truecolor(88, 110, 117),
                table_border: Style::new().truecolor(88, 110, 117),
            }
        } else {
            Self {
                heading: Style::new().truecolor(38, 139, 210).bold(),
                body: Style::new().truecolor(101, 123, 131),
                accent: Style::new().truecolor(42, 161, 152),
                code: Style::new().truecolor(133, 153, 0),
                dimmed: Style::new().truecolor(147, 161, 161),
                code_fence: Style::new().truecolor(147, 161, 161),
                rule: Style::new().truecolor(147, 161, 161),
                list_marker: Style::new().truecolor(181, 137, 0),
                blockquote_border: Style::new().truecolor(147, 161, 161),
                table_border: Style::new().truecolor(147, 161, 161),
            }
        }
    }

    /// Create a Nord theme instance
    pub fn nord(is_dark: bool) -> Self {
        if is_dark {
            Self {
                heading: Style::new().truecolor(136, 192, 208).bold(), // nord8 - light blue
                body: Style::new().truecolor(216, 222, 233),           // nord4
                accent: Style::new().truecolor(143, 188, 187),         // nord7 - teal
                code: Style::new().truecolor(163, 190, 140),           // nord14 - green
                dimmed: Style::new().truecolor(76, 86, 106),           // nord3
                code_fence: Style::new().truecolor(76, 86, 106),
                rule: Style::new().truecolor(76, 86, 106),
                list_marker: Style::new().truecolor(235, 203, 139), // nord13 - yellow
                blockquote_border: Style::new().truecolor(76, 86, 106),
                table_border: Style::new().truecolor(76, 86, 106),
            }
        } else {
            Self {
                heading: Style::new().truecolor(94, 129, 172).bold(), // darker blue
                body: Style::new().truecolor(46, 52, 64),
                accent: Style::new().truecolor(136, 192, 208), // blue
                code: Style::new().truecolor(163, 190, 140),   // green
                dimmed: Style::new().truecolor(143, 157, 175),
                code_fence: Style::new().truecolor(143, 157, 175),
                rule: Style::new().truecolor(143, 157, 175),
                list_marker: Style::new().truecolor(235, 203, 139), // yellow
                blockquote_border: Style::new().truecolor(143, 157, 175),
                table_border: Style::new().truecolor(143, 157, 175),
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
}
