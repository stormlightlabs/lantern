//! Error types shared by parsing, validation, theme loading, and rendering helpers.

use std::io;
use thiserror::Error;

/// Errors that can occur during slide parsing and rendering
#[derive(Error, Debug)]
pub enum SlideError {
    /// Failure while reading from or writing to the filesystem.
    #[error("Failed to read file: {0}")]
    IoError(#[from] io::Error),

    /// Markdown parsing failed at a specific source line.
    #[error("Failed to parse markdown at line {line}: {message}")]
    ParseError {
        /// One-based source line where parsing failed.
        line: usize,
        /// Human-readable parse failure.
        message: String,
    },

    /// Slide content does not match a supported lantern format.
    #[error("Invalid slide format: {0}")]
    InvalidFormat(String),

    /// YAML or TOML front matter could not be extracted or decoded.
    #[error("Front matter error: {0}")]
    FrontMatterError(String),

    /// YAML decoding failed.
    #[error("YAML parsing failed: {0}")]
    YamlError(#[from] yaml_serde::Error),

    /// JSON decoding failed.
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Theme lookup, parsing, or validation failed.
    #[error("Theme validation error: {0}")]
    ThemeError(String),
}

/// Result alias used by lantern parsing, validation, and theme helpers.
pub type Result<T> = std::result::Result<T, SlideError>;

impl SlideError {
    /// Create a markdown parse error with a source line number.
    pub fn parse_error(line: usize, message: impl Into<String>) -> Self {
        Self::ParseError { line, message: message.into() }
    }

    /// Create an invalid slide format error.
    pub fn invalid_format(message: impl Into<String>) -> Self {
        Self::InvalidFormat(message.into())
    }

    /// Create a front matter parsing error.
    pub fn front_matter(message: impl Into<String>) -> Self {
        Self::FrontMatterError(message.into())
    }

    /// Create a theme loading or validation error.
    pub fn theme_error(message: impl Into<String>) -> Self {
        Self::ThemeError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_creation() {
        let err = SlideError::parse_error(10, "Invalid syntax");
        assert!(err.to_string().contains("line 10"));
        assert!(err.to_string().contains("Invalid syntax"));
    }

    #[test]
    fn error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let slide_err: SlideError = io_err.into();
        assert!(slide_err.to_string().contains("Failed to read file"));
    }
}
