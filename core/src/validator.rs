use crate::error::{Result, SlideError};
use crate::metadata::Meta;
use crate::parser::parse_slides_with_meta;
use crate::theme::{Base16Scheme, ThemeColors, ThemeRegistry};

use std::path::Path;

/// Validation result containing errors and warnings
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_issues(&self) -> bool {
        !self.errors.is_empty() || !self.warnings.is_empty()
    }
}

/// Validate a slide deck markdown file
///
/// Checks for:
/// - File readability
/// - Valid frontmatter (YAML/TOML)
/// - Slide parsing
/// - Empty slide deck
/// - Theme references
pub fn validate_slides(file_path: &Path, strict: bool) -> ValidationResult {
    let mut result = ValidationResult::new();

    let markdown = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            result.add_error(format!("Failed to read file '{}': {}", file_path.display(), e));
            return result;
        }
    };

    let (meta, slides) = match parse_slides_with_meta(&markdown) {
        Ok((m, s)) => (m, s),
        Err(e) => {
            result.add_error(format!("Parse error: {e}"));
            return result;
        }
    };

    if slides.is_empty() {
        result.add_error("No slides found in file".to_string());
        return result;
    }

    if strict {
        validate_metadata(&meta, &mut result);
        validate_slide_content(&slides, &mut result);
    }

    result
}

/// Validate metadata fields
fn validate_metadata(meta: &Meta, result: &mut ValidationResult) {
    if meta.theme != "default" && !ThemeRegistry::available_themes().contains(&meta.theme.as_str()) {
        result.add_warning(format!(
            "Theme '{}' is not a built-in theme. Available themes: {}",
            meta.theme,
            ThemeRegistry::available_themes().join(", ")
        ));
    }

    if meta.author == "Unknown" {
        result.add_warning("No author specified in frontmatter".to_string());
    }
}

/// Validate slide content
fn validate_slide_content(slides: &[crate::slide::Slide], result: &mut ValidationResult) {
    for (idx, slide) in slides.iter().enumerate() {
        if slide.blocks.is_empty() {
            result.add_warning(format!("Slide {} is empty", idx + 1));
        }
    }
}

/// Validate a theme file
///
/// Checks for:
/// - File readability
/// - Valid YAML format
/// - Base16 schema compliance
/// - Color format validity
pub fn validate_theme_file(file_path: &Path) -> ValidationResult {
    let mut result = ValidationResult::new();

    let yaml_content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            result.add_error(format!("Failed to read theme file '{}': {}", file_path.display(), e));
            return result;
        }
    };

    let scheme: Base16Scheme = match serde_yml::from_str(&yaml_content) {
        Ok(s) => s,
        Err(e) => {
            result.add_error(format!("Failed to parse YAML: {e}"));
            return result;
        }
    };

    validate_base16_scheme(&scheme, &mut result);

    if result.is_valid() {
        let colors = vec![
            ("base00", &scheme.palette.base00),
            ("base01", &scheme.palette.base01),
            ("base02", &scheme.palette.base02),
            ("base03", &scheme.palette.base03),
            ("base04", &scheme.palette.base04),
            ("base05", &scheme.palette.base05),
            ("base06", &scheme.palette.base06),
            ("base07", &scheme.palette.base07),
            ("base08", &scheme.palette.base08),
            ("base09", &scheme.palette.base09),
            ("base0A", &scheme.palette.base0a),
            ("base0B", &scheme.palette.base0b),
            ("base0C", &scheme.palette.base0c),
            ("base0D", &scheme.palette.base0d),
            ("base0E", &scheme.palette.base0e),
            ("base0F", &scheme.palette.base0f),
        ];

        for (name, color) in colors {
            validate_hex_color(name, color, &mut result);
        }
    }

    result
}

/// Validate base16 scheme structure
fn validate_base16_scheme(scheme: &Base16Scheme, result: &mut ValidationResult) {
    if scheme.system != "base16" {
        result.add_error(format!("Invalid system '{}', expected 'base16'", scheme.system));
    }

    if scheme.name.is_empty() {
        result.add_error("Theme name is empty".to_string());
    }

    if scheme.author.is_empty() {
        result.add_warning("Theme author is empty".to_string());
    }

    let valid_variants = ["dark", "light"];
    if !valid_variants.contains(&scheme.variant.as_str()) {
        result.add_warning(format!("Variant '{}' should be 'dark' or 'light'", scheme.variant));
    }
}

/// Validate hex color format
fn validate_hex_color(name: &str, hex: &str, result: &mut ValidationResult) {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        result.add_error(format!(
            "Color {} has invalid length {} (expected 6 hex digits)",
            name,
            hex.len()
        ));
        return;
    }

    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        result.add_error(format!("Color {name} contains invalid hex characters"));
    }
}

/// Validate theme by name
///
/// Checks if the theme exists in the built-in registry
pub fn validate_theme_name(name: &str) -> Result<ThemeColors> {
    let available = ThemeRegistry::available_themes();

    if available.contains(&name) || name == "default" {
        Ok(ThemeRegistry::get(name))
    } else {
        Err(SlideError::theme_error(format!(
            "Theme '{}' not found. Available themes: {}",
            name,
            available.join(", ")
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_slides_nonexistent_file() {
        let path = Path::new("/nonexistent/file.md");
        let result = validate_slides(path, false);
        assert!(!result.is_valid());
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].contains("Failed to read file"));
    }

    #[test]
    fn validate_slides_empty_content() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_empty_validation.md");
        std::fs::write(&test_file, "").expect("Failed to write test file");

        let result = validate_slides(&test_file, false);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.contains("No slides found")));

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_slides_valid_content() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_valid_validation.md");
        let content = "# Test Slide\n\nThis is a test paragraph.";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = validate_slides(&test_file, false);
        assert!(result.is_valid());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_slides_invalid_frontmatter() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_invalid_frontmatter.md");
        let content = "---\ninvalid yaml: [unclosed\n---\n# Slide";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = validate_slides(&test_file, false);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.contains("Parse error")));

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_slides_with_warnings_strict() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_warnings_validation.md");
        let content = "---\ntheme: nonexistent-theme\nauthor: Unknown\n---\n# Slide 1\n\nContent";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = validate_slides(&test_file, true);
        assert!(result.is_valid());
        assert!(!result.warnings.is_empty());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_theme_file_invalid_yaml() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_invalid_theme.yml");
        let content = "invalid: yaml: content: [unclosed";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = validate_theme_file(&test_file);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.contains("Failed to parse YAML")));

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_theme_file_invalid_system() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_invalid_system.yml");
        let content = r###"
system: "base32"
name: "Test"
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
"###;
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = validate_theme_file(&test_file);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.contains("Invalid system") && e.contains("base32"))
        );

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_theme_file_invalid_color() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_invalid_color.yml");
        let content = r###"
system: "base16"
name: "Test"
author: "Test Author"
variant: "dark"
palette:
  base00: "#000000"
  base01: "#111111"
  base02: "#222222"
  base03: "#333333"
  base04: "#GGGGGG"
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
"###;
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = validate_theme_file(&test_file);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.contains("base04") && e.contains("invalid hex"))
        );

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_theme_file_valid() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_valid_theme.yml");
        let content = r###"
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
"###;
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = validate_theme_file(&test_file);
        assert!(result.is_valid());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn validate_theme_name_builtin() {
        let result = validate_theme_name("nord");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_theme_name_default() {
        let result = validate_theme_name("default");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_theme_name_invalid() {
        let result = validate_theme_name("nonexistent-theme");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Theme 'nonexistent-theme' not found")
        );
    }

    #[test]
    fn validation_result_is_valid() {
        let mut result = ValidationResult::new();
        assert!(result.is_valid());

        result.add_warning("test warning".to_string());
        assert!(result.is_valid());

        result.add_error("test error".to_string());
        assert!(!result.is_valid());
    }

    #[test]
    fn validation_result_has_issues() {
        let mut result = ValidationResult::new();
        assert!(!result.has_issues());

        result.add_warning("test warning".to_string());
        assert!(result.has_issues());
    }
}
