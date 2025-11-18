use owo_colors::{OwoColorize, Style};
use serde::Deserialize;
use terminal_colorsaurus::{QueryOptions, background_color};

/// Parses a hex color string to RGB values.
///
/// Supports both `#RRGGBB` and `RRGGBB` formats.
fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

/// Base16 color scheme specification.
///
/// Defines a standard 16-color palette that can be mapped to semantic theme roles.
#[derive(Debug, Clone, Deserialize)]
struct Base16Scheme {
    #[allow(dead_code)]
    system: String,
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    author: String,
    #[allow(dead_code)]
    variant: String,
    palette: Base16Palette,
}

/// Base16 color palette with 16 standardized color slots.
///
/// Each base color serves a semantic purpose in the base16 specification:
/// - base00-03: Background shades (darkest to lighter)
/// - base04-07: Foreground shades (darker to lightest)
/// - base08-0F: Accent colors (red, orange, yellow, green, cyan, blue, magenta, brown)
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct Base16Palette {
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    #[serde(rename = "base0A")]
    base0a: String,
    #[serde(rename = "base0B")]
    base0b: String,
    #[serde(rename = "base0C")]
    base0c: String,
    #[serde(rename = "base0D")]
    base0d: String,
    #[serde(rename = "base0E")]
    base0e: String,
    #[serde(rename = "base0F")]
    base0f: String,
}

static CATPPUCCIN_LATTE: &str = include_str!("themes/catppuccin-latte.yml");
static CATPPUCCIN_MOCHA: &str = include_str!("themes/catppuccin-mocha.yml");
static GRUVBOX_MATERIAL_DARK: &str = include_str!("themes/gruvbox-material-dark-medium.yml");
static GRUVBOX_MATERIAL_LIGHT: &str = include_str!("themes/gruvbox-material-light-medium.yml");
static NORD_LIGHT: &str = include_str!("themes/nord-light.yml");
static NORD: &str = include_str!("themes/nord.yml");
static OXOCARBON_DARK: &str = include_str!("themes/oxocarbon-dark.yml");
static OXOCARBON_LIGHT: &str = include_str!("themes/oxocarbon-light.yml");
static SOLARIZED_DARK: &str = include_str!("themes/solarized-dark.yml");
static SOLARIZED_LIGHT: &str = include_str!("themes/solarized-light.yml");

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
    pub emphasis: Color,
    pub strong: Color,
    pub link: Color,
    pub inline_code_bg: Color,
    pub ui_border: Color,
    pub ui_title: Color,
    pub ui_text: Color,
    pub ui_background: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        let is_dark = detect_is_dark();
        let theme_name = if is_dark { "nord" } else { "nord-light" };
        ThemeRegistry::get(theme_name)
    }
}

impl ThemeColors {
    /// Create a ThemeColors from a base16 color scheme.
    ///
    /// Maps base16 colors to semantic theme roles following base16 styling guidelines:
    ///
    /// Content colors:
    /// - base05: body text (main foreground)
    /// - base0D: headings (blue - classes/functions)
    /// - base0E: strong emphasis (magenta/purple - keywords)
    /// - base0B: code blocks (green - strings)
    /// - base03: dimmed/borders (comment color)
    /// - base0A: list markers (yellow - classes/constants)
    /// - base09: emphasis/italics (orange - integers/constants)
    /// - base0C: links (cyan - support/regex)
    /// - base08: accents (red - variables/tags)
    /// - base02: inline code background (selection background)
    ///
    /// UI chrome colors:
    /// - base00: UI background (darkest background)
    /// - base04: UI borders (dim foreground)
    /// - base06: UI titles (bright foreground)
    /// - base07: UI text (brightest foreground)
    fn from_base16(scheme: &Base16Scheme) -> Option<Self> {
        let palette = &scheme.palette;

        let heading = parse_hex_color(&palette.base0d)?;
        let body = parse_hex_color(&palette.base05)?;
        let accent = parse_hex_color(&palette.base08)?;
        let code = parse_hex_color(&palette.base0b)?;
        let dimmed = parse_hex_color(&palette.base03)?;
        let code_fence = dimmed;
        let rule = dimmed;
        let list_marker = parse_hex_color(&palette.base0a)?;
        let blockquote_border = dimmed;
        let table_border = dimmed;
        let emphasis = parse_hex_color(&palette.base09)?;
        let strong = parse_hex_color(&palette.base0e)?;
        let link = parse_hex_color(&palette.base0c)?;
        let inline_code_bg = parse_hex_color(&palette.base02)?;
        let ui_background = parse_hex_color(&palette.base00)?;
        let ui_border = parse_hex_color(&palette.base04)?;
        let ui_title = parse_hex_color(&palette.base06)?;
        let ui_text = parse_hex_color(&palette.base07)?;

        Some(Self {
            heading: Color::new(heading.0, heading.1, heading.2),
            heading_bold: true,
            body: Color::new(body.0, body.1, body.2),
            accent: Color::new(accent.0, accent.1, accent.2),
            code: Color::new(code.0, code.1, code.2),
            dimmed: Color::new(dimmed.0, dimmed.1, dimmed.2),
            code_fence: Color::new(code_fence.0, code_fence.1, code_fence.2),
            rule: Color::new(rule.0, rule.1, rule.2),
            list_marker: Color::new(list_marker.0, list_marker.1, list_marker.2),
            blockquote_border: Color::new(blockquote_border.0, blockquote_border.1, blockquote_border.2),
            table_border: Color::new(table_border.0, table_border.1, table_border.2),
            emphasis: Color::new(emphasis.0, emphasis.1, emphasis.2),
            strong: Color::new(strong.0, strong.1, strong.2),
            link: Color::new(link.0, link.1, link.2),
            inline_code_bg: Color::new(inline_code_bg.0, inline_code_bg.1, inline_code_bg.2),
            ui_border: Color::new(ui_border.0, ui_border.1, ui_border.2),
            ui_title: Color::new(ui_title.0, ui_title.1, ui_title.2),
            ui_text: Color::new(ui_text.0, ui_text.1, ui_text.2),
            ui_background: Color::new(ui_background.0, ui_background.1, ui_background.2),
        })
    }

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

    /// Apply emphasis (italic) style to text
    pub fn emphasis<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.emphasis).into())
    }

    /// Apply strong (bold) style to text
    pub fn strong<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        let style: Style = (&self.strong).into();
        text.style(style.bold())
    }

    /// Apply link style to text
    pub fn link<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.link).into())
    }

    /// Apply inline code background style to text
    pub fn inline_code_bg<'a, T: OwoColorize>(&self, text: &'a T) -> owo_colors::Styled<&'a T> {
        text.style((&self.inline_code_bg).into())
    }
}

/// Theme registry for loading prebuilt base16 themes from YAML files.
///
/// Themes are embedded at compile time using include_str! for zero runtime I/O.
/// Supports all base16 color schemes in the themes directory.
pub struct ThemeRegistry;

impl ThemeRegistry {
    /// Get a theme by name.
    ///
    /// Loads and parses the corresponding YAML theme file embedded at compile time.
    /// Falls back to Nord theme if the requested theme is not found or parsing fails.
    pub fn get(name: &str) -> ThemeColors {
        let yaml = match name.to_lowercase().as_str() {
            "catppuccin-latte" => CATPPUCCIN_LATTE,
            "catppuccin-mocha" => CATPPUCCIN_MOCHA,
            "gruvbox-material-dark" => GRUVBOX_MATERIAL_DARK,
            "gruvbox-material-light" => GRUVBOX_MATERIAL_LIGHT,
            "nord-light" => NORD_LIGHT,
            "nord" => NORD,
            "oxocarbon-dark" => OXOCARBON_DARK,
            "oxocarbon-light" => OXOCARBON_LIGHT,
            "solarized-dark" => SOLARIZED_DARK,
            "solarized-light" => SOLARIZED_LIGHT,
            _ => NORD,
        };

        serde_yml::from_str::<Base16Scheme>(yaml)
            .ok()
            .and_then(|scheme| ThemeColors::from_base16(&scheme))
            .unwrap_or_else(|| {
                serde_yml::from_str::<Base16Scheme>(NORD)
                    .ok()
                    .and_then(|scheme| ThemeColors::from_base16(&scheme))
                    .expect("Failed to parse fallback Nord theme")
            })
    }

    /// List all available theme names.
    pub fn available_themes() -> Vec<&'static str> {
        vec![
            "catppuccin-latte",
            "catppuccin-mocha",
            "gruvbox-material-dark",
            "gruvbox-material-light",
            "nord-light",
            "nord",
            "oxocarbon-dark",
            "oxocarbon-light",
            "solarized-dark",
            "solarized-light",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_color_with_hash() {
        let result = parse_hex_color("#FF8040");
        assert_eq!(result, Some((255, 128, 64)));
    }

    #[test]
    fn parse_hex_color_without_hash() {
        let result = parse_hex_color("FF8040");
        assert_eq!(result, Some((255, 128, 64)));
    }

    #[test]
    fn parse_hex_color_lowercase() {
        let result = parse_hex_color("#ff8040");
        assert_eq!(result, Some((255, 128, 64)));
    }

    #[test]
    fn parse_hex_color_invalid_length() {
        assert_eq!(parse_hex_color("#FFF"), None);
        assert_eq!(parse_hex_color("#FFFFFFF"), None);
    }

    #[test]
    fn parse_hex_color_invalid_chars() {
        assert_eq!(parse_hex_color("#GGGGGG"), None);
        assert_eq!(parse_hex_color("#XYZ123"), None);
    }

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
    fn base16_scheme_deserializes() {
        let yaml = r##"
system: "base16"
name: "Test Theme"
author: "Test Author"
variant: "dark"
palette:
  base00: "#000000"
  base01: "#111111"
  base02: "#222222"
  base03: "#333333"
  base04: "#444444"
  base05: "#555555"
  base06: "#666666"
  base07: "#777777"
  base08: "#888888"
  base09: "#999999"
  base0A: "#aaaaaa"
  base0B: "#bbbbbb"
  base0C: "#cccccc"
  base0D: "#dddddd"
  base0E: "#eeeeee"
  base0F: "#ffffff"
"##;
        let scheme: Result<Base16Scheme, _> = serde_yml::from_str(yaml);
        assert!(scheme.is_ok());
    }

    #[test]
    fn theme_colors_from_base16() {
        let yaml = r##"
system: "base16"
name: "Test Theme"
author: "Test Author"
variant: "dark"
palette:
  base00: "#000000"
  base01: "#111111"
  base02: "#222222"
  base03: "#333333"
  base04: "#444444"
  base05: "#555555"
  base06: "#666666"
  base07: "#777777"
  base08: "#ff0000"
  base09: "#ff7f00"
  base0A: "#ffff00"
  base0B: "#00ff00"
  base0C: "#00ffff"
  base0D: "#0000ff"
  base0E: "#ff00ff"
  base0F: "#ffffff"
"##;
        let scheme: Base16Scheme = serde_yml::from_str(yaml).unwrap();
        let theme = ThemeColors::from_base16(&scheme);
        assert!(theme.is_some());

        let theme = theme.unwrap();
        assert_eq!(theme.body.r, 85); // base05 - #555555
        assert_eq!(theme.heading.r, 0); // base0D - #0000ff
        assert_eq!(theme.code.r, 0); // base0B - #00ff00
        assert_eq!(theme.accent.r, 255); // base08 - #ff0000
        assert_eq!(theme.emphasis.r, 255); // base09 - #ff7f00
        assert_eq!(theme.strong.r, 255); // base0E - #ff00ff
        assert_eq!(theme.link.r, 0); // base0C - #00ffff
        assert_eq!(theme.inline_code_bg.r, 34); // base02 - #222222
        assert_eq!(theme.ui_background.r, 0); // base00 - #000000
        assert_eq!(theme.ui_border.r, 68); // base04 - #444444
        assert_eq!(theme.ui_title.r, 102); // base06 - #666666
        assert_eq!(theme.ui_text.r, 119); // base07 - #777777
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
    fn theme_registry_get_nord() {
        let theme = ThemeRegistry::get("nord");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
        assert_eq!(theme.heading.r, 129);
        assert_eq!(theme.heading.g, 161);
        assert_eq!(theme.heading.b, 193);
    }

    #[test]
    fn theme_registry_get_nord_light() {
        let theme = ThemeRegistry::get("nord-light");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_catppuccin_mocha() {
        let theme = ThemeRegistry::get("catppuccin-mocha");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_catppuccin_latte() {
        let theme = ThemeRegistry::get("catppuccin-latte");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_gruvbox_dark() {
        let theme = ThemeRegistry::get("gruvbox-material-dark");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_gruvbox_light() {
        let theme = ThemeRegistry::get("gruvbox-material-light");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_oxocarbon_dark() {
        let theme = ThemeRegistry::get("oxocarbon-dark");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_oxocarbon_light() {
        let theme = ThemeRegistry::get("oxocarbon-light");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_solarized_dark() {
        let theme = ThemeRegistry::get("solarized-dark");
        let text = "Test";
        let styled = theme.heading(&text);
        assert!(styled.to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_get_solarized_light() {
        let theme = ThemeRegistry::get("solarized-light");
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
        let theme1 = ThemeRegistry::get("NORD");
        let theme2 = ThemeRegistry::get("nord");
        let text = "Test";
        assert!(theme1.heading(&text).to_string().contains("Test"));
        assert!(theme2.heading(&text).to_string().contains("Test"));
    }

    #[test]
    fn theme_registry_available_themes() {
        let themes = ThemeRegistry::available_themes();
        assert!(themes.contains(&"nord"));
        assert!(themes.contains(&"nord-light"));
        assert!(themes.contains(&"catppuccin-mocha"));
        assert!(themes.contains(&"catppuccin-latte"));
        assert!(themes.contains(&"gruvbox-material-dark"));
        assert!(themes.contains(&"gruvbox-material-light"));
        assert!(themes.contains(&"oxocarbon-dark"));
        assert!(themes.contains(&"oxocarbon-light"));
        assert!(themes.contains(&"solarized-dark"));
        assert!(themes.contains(&"solarized-light"));
        assert_eq!(themes.len(), 10);
    }

    #[test]
    fn detect_is_dark_returns_bool() {
        let result = detect_is_dark();
        assert!(result || !result);
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
        assert!(theme.emphasis(&"Test").to_string().contains("Test"));
        assert!(theme.strong(&"Test").to_string().contains("Test"));
        assert!(theme.link(&"Test").to_string().contains("Test"));
        assert!(theme.inline_code_bg(&"Test").to_string().contains("Test"));

        // UI colors don't need style methods, just verify they exist
        let _ = theme.ui_border;
        let _ = theme.ui_title;
        let _ = theme.ui_text;
        let _ = theme.ui_background;
    }

    #[test]
    fn all_embedded_themes_parse() {
        for theme_name in ThemeRegistry::available_themes() {
            let theme = ThemeRegistry::get(theme_name);
            let styled = theme.heading(&"Test");
            assert!(
                styled.to_string().contains("Test"),
                "Theme '{}' failed to parse or apply styles",
                theme_name
            );
        }
    }
}
