use crate::error::{Result, SlideError};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::SystemTime;

/// Slide deck metadata from YAML frontmatter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default = "Meta::default_theme")]
    pub theme: String,
    #[serde(default = "Meta::default_author")]
    pub author: String,
    #[serde(default = "Meta::default_date")]
    pub date: String,
    #[serde(default = "Meta::default_paging")]
    pub paging: String,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            theme: Self::default_theme(),
            author: Self::default_author(),
            date: Self::default_date(),
            paging: Self::default_paging(),
        }
    }
}

impl Meta {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse metadata from YAML or TOML frontmatter header
    fn parse(header: &str, format: FrontmatterFormat) -> Result<Self> {
        if header.trim().is_empty() {
            return Ok(Self::default());
        }

        match format {
            FrontmatterFormat::Yaml => match serde_yml::from_str(header) {
                Ok(meta) => Ok(meta),
                Err(e) => Err(SlideError::front_matter(format!("Failed to parse YAML: {}", e))),
            },
            FrontmatterFormat::Toml => match toml::from_str(header) {
                Ok(meta) => Ok(meta),
                Err(e) => Err(SlideError::front_matter(format!("Failed to parse TOML: {}", e))),
            },
        }
    }

    /// Extract frontmatter block with the given delimiter and format
    fn extract_frontmatter(rest: &str, delimiter: &str, format: FrontmatterFormat) -> Result<(Self, String)> {
        match rest.find(&format!("\n{}", delimiter)) {
            Some(end_pos) => Ok((
                Self::parse(&rest[..end_pos], format)?,
                rest[end_pos + delimiter.len() + 1..].to_string(),
            )),
            None => Err(SlideError::front_matter(format!(
                "Unclosed {} frontmatter block (missing closing {})",
                format, delimiter
            ))),
        }
    }

    /// Extract metadata and content from markdown
    pub fn extract_from_markdown(markdown: &str) -> Result<(Self, String)> {
        let trimmed = markdown.trim_start();
        match trimmed.chars().take(3).collect::<String>().as_str() {
            "---" => Self::extract_frontmatter(&trimmed[3..], "---", FrontmatterFormat::Yaml),
            "+++" => Self::extract_frontmatter(&trimmed[3..], "+++", FrontmatterFormat::Toml),
            _ => Ok((Self::default(), markdown.to_string())),
        }
    }

    /// Get theme from environment variable or return "default"
    fn default_theme() -> String {
        env::var("SLIDES_THEME").unwrap_or_else(|_| "default".to_string())
    }

    /// Get current system user's name
    fn default_author() -> String {
        env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "Unknown".to_string())
    }

    /// Get current date in YYYY-MM-DD format
    fn default_date() -> String {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                let days = duration.as_secs() / 86400;
                let epoch_days = days as i64;
                let year = 1970 + (epoch_days / 365);

                let day_of_year = epoch_days % 365;
                let month = (day_of_year / 30) + 1;
                let day = (day_of_year % 30) + 1;
                format!("{:04}-{:02}-{:02}", year, month, day)
            }
            Err(_) => "Unknown".to_string(),
        }
    }

    /// Default paging format
    fn default_paging() -> String {
        "Slide %d / %d".to_string()
    }
}

/// Frontmatter format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FrontmatterFormat {
    Yaml,
    Toml,
}

impl std::fmt::Display for FrontmatterFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FrontmatterFormat::Yaml => "YAML",
                FrontmatterFormat::Toml => "TOML",
            }
            .to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_default() {
        let meta = Meta::default();
        assert_eq!(meta.paging, "Slide %d / %d");
        assert!(!meta.theme.is_empty());
    }

    #[test]
    fn meta_parse_yaml_empty() {
        let meta = Meta::parse("", FrontmatterFormat::Yaml).unwrap();
        assert_eq!(meta, Meta::default());
    }

    #[test]
    fn meta_parse_yaml_partial() {
        let yaml = "theme: dark\nauthor: Test Author";
        let meta = Meta::parse(yaml, FrontmatterFormat::Yaml).unwrap();
        assert_eq!(meta.theme, "dark");
        assert_eq!(meta.author, "Test Author");
        assert_eq!(meta.paging, "Slide %d / %d");
    }

    #[test]
    fn meta_parse_yaml_full() {
        let yaml = r#"
theme: monokai
author: John Doe
date: 2024-01-15
paging: "Page %d of %d"
        "#;
        let meta = Meta::parse(yaml, FrontmatterFormat::Yaml).unwrap();
        assert_eq!(meta.theme, "monokai");
        assert_eq!(meta.author, "John Doe");
        assert_eq!(meta.date, "2024-01-15");
        assert_eq!(meta.paging, "Page %d of %d");
    }

    #[test]
    fn meta_parse_toml() {
        let toml = r#"
theme = "dracula"
author = "Jane Doe"
date = "2024-01-20"
paging = "Slide %d of %d"
        "#;
        let meta = Meta::parse(toml, FrontmatterFormat::Toml).unwrap();
        assert_eq!(meta.theme, "dracula");
        assert_eq!(meta.author, "Jane Doe");
        assert_eq!(meta.date, "2024-01-20");
        assert_eq!(meta.paging, "Slide %d of %d");
    }

    #[test]
    fn extract_frontmatter() {
        let markdown = r#"---
theme: dark
author: Test
---
# First Slide
Content here"#;

        let (meta, content) = Meta::extract_from_markdown(markdown).unwrap();
        assert_eq!(meta.theme, "dark");
        assert_eq!(meta.author, "Test");
        assert!(content.contains("# First Slide"));
    }

    #[test]
    fn extract_no_frontmatter() {
        let markdown = "# First Slide\nContent";
        let (meta, content) = Meta::extract_from_markdown(markdown).unwrap();
        assert_eq!(meta, Meta::default());
        assert_eq!(content, markdown);
    }

    #[test]
    fn extract_unclosed_yaml_frontmatter() {
        let markdown = "---\ntheme: dark\n# Slide";
        let result = Meta::extract_from_markdown(markdown);
        assert!(result.is_err());
    }

    #[test]
    fn extract_toml_frontmatter() {
        let markdown = r#"+++
theme = "dark"
author = "Test"
+++
# First Slide
Content here"#;

        let (meta, content) = Meta::extract_from_markdown(markdown).unwrap();
        assert_eq!(meta.theme, "dark");
        assert_eq!(meta.author, "Test");
        assert!(content.contains("# First Slide"));
    }

    #[test]
    fn extract_unclosed_toml_frontmatter() {
        let markdown = "+++\ntheme = \"dark\"\n# Slide";
        let result = Meta::extract_from_markdown(markdown);
        assert!(result.is_err());
    }
}
