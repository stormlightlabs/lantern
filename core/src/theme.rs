use owo_colors::{OwoColorize, Style};

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
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            heading: Style::new().bright_cyan().bold(),
            body: Style::new().white(),
            accent: Style::new().bright_yellow(),
            code: Style::new().bright_green(),
            dimmed: Style::new().dimmed(),
        }
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
}
