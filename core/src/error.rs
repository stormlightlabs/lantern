use std::io;
use thiserror::Error;

/// Errors that can occur during slide parsing and rendering
#[derive(Error, Debug)]
pub enum SlideError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to parse markdown at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("Invalid slide format: {0}")]
    InvalidFormat(String),

    #[error("Front matter error: {0}")]
    FrontMatterError(String),

    #[error("YAML parsing failed: {0}")]
    YamlError(#[from] serde_yml::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Theme validation error: {0}")]
    ThemeError(String),
}

pub type Result<T> = std::result::Result<T, SlideError>;

impl SlideError {
    pub fn parse_error(line: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            message: message.into(),
        }
    }

    pub fn invalid_format(message: impl Into<String>) -> Self {
        Self::InvalidFormat(message.into())
    }

    pub fn front_matter(message: impl Into<String>) -> Self {
        Self::FrontMatterError(message.into())
    }

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
