use lantern_core::{slide::Slide, theme::ThemeColors};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};
use std::time::Instant;

use crate::renderer::render_slide_content;

#[derive(Clone, Copy)]
struct Stylesheet {
    theme: ThemeColors,
}

impl Stylesheet {
    fn new(theme: ThemeColors) -> Self {
        Self { theme }
    }

    fn slide_padding() -> Padding {
        Padding::new(4, 4, 2, 2)
    }

    fn status_bar(&self) -> Style {
        Style::default()
            .bg(self.bg_color())
            .fg(self.ui_text_color())
            .add_modifier(Modifier::BOLD)
    }

    fn border_color(&self) -> Color {
        Color::Rgb(self.theme.ui_border.r, self.theme.ui_border.g, self.theme.ui_border.b)
    }

    fn title_color(&self) -> Color {
        Color::Rgb(self.theme.ui_title.r, self.theme.ui_title.g, self.theme.ui_title.b)
    }

    fn text_color(&self) -> Color {
        Color::Rgb(self.theme.body.r, self.theme.body.g, self.theme.body.b)
    }

    fn bg_color(&self) -> Color {
        Color::Rgb(
            self.theme.ui_background.r,
            self.theme.ui_background.g,
            self.theme.ui_background.b,
        )
    }

    fn ui_text_color(&self) -> Color {
        Color::Rgb(self.theme.ui_text.r, self.theme.ui_text.g, self.theme.ui_text.b)
    }
}

impl From<ThemeColors> for Stylesheet {
    fn from(value: ThemeColors) -> Self {
        Self::new(value)
    }
}

/// Slide viewer state manager
///
/// Manages current slide index, navigation, and speaker notes visibility.
pub struct SlideViewer {
    slides: Vec<Slide>,
    current_index: usize,
    show_notes: bool,
    filename: Option<String>,
    stylesheet: Stylesheet,
    theme_name: String,
    start_time: Option<Instant>,
}

impl SlideViewer {
    /// Create a new slide viewer with slides and theme
    pub fn new(slides: Vec<Slide>, theme: ThemeColors) -> Self {
        Self {
            slides,
            current_index: 0,
            show_notes: false,
            stylesheet: theme.into(),
            filename: None,
            theme_name: "default".to_string(),
            start_time: None,
        }
    }

    /// Create a slide viewer with full presentation context
    pub fn with_context(
        slides: Vec<Slide>, theme: ThemeColors, filename: Option<String>, theme_name: String,
        start_time: Option<Instant>,
    ) -> Self {
        Self {
            slides,
            current_index: 0,
            show_notes: false,
            stylesheet: theme.into(),
            filename,
            theme_name,
            start_time,
        }
    }

    /// Navigate to the next slide
    pub fn next(&mut self) {
        if self.current_index < self.slides.len().saturating_sub(1) {
            self.current_index += 1;
        }
    }

    /// Navigate to the previous slide
    pub fn previous(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    /// Jump to a specific slide by index (0-based)
    pub fn jump_to(&mut self, index: usize) {
        if index < self.slides.len() {
            self.current_index = index;
        }
    }

    /// Toggle speaker notes visibility
    pub fn toggle_notes(&mut self) {
        self.show_notes = !self.show_notes;
    }

    /// Get the current slide
    pub fn current_slide(&self) -> Option<&Slide> {
        self.slides.get(self.current_index)
    }

    /// Get the current slide index (0-based)
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Get total number of slides
    pub fn total_slides(&self) -> usize {
        self.slides.len()
    }

    /// Check if speaker notes are visible
    pub fn is_showing_notes(&self) -> bool {
        self.show_notes
    }

    fn theme(&self) -> ThemeColors {
        self.stylesheet.theme.clone()
    }

    /// Render the current slide to the frame
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if let Some(slide) = self.current_slide() {
            let content = render_slide_content(&slide.blocks, &self.theme());
            let border_color = self.stylesheet.border_color();
            let title_color = self.stylesheet.title_color();

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(format!(" Slide {}/{} ", self.current_index + 1, self.total_slides()))
                .title_style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
                .padding(Stylesheet::slide_padding());

            let paragraph = Paragraph::new(content).block(block).wrap(Wrap { trim: false });

            frame.render_widget(paragraph, area);
        }
    }

    /// Render speaker notes if available and visible
    pub fn render_notes(&self, frame: &mut Frame, area: Rect) {
        if !self.show_notes {
            return;
        }

        if let Some(slide) = self.current_slide() {
            if let Some(notes) = &slide.notes {
                let border_color = self.stylesheet.border_color();
                let title_color = self.stylesheet.title_color();
                let text_color = self.stylesheet.text_color();

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
                    .title(" Speaker Notes ")
                    .title_style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
                    .padding(Stylesheet::slide_padding());

                let paragraph = Paragraph::new(notes.clone())
                    .block(block)
                    .wrap(Wrap { trim: false })
                    .style(Style::default().fg(text_color));

                frame.render_widget(paragraph, area);
            }
        }
    }

    /// Render status bar with navigation info
    pub fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let filename_part = self.filename.as_ref().map(|f| format!("{} | ", f)).unwrap_or_default();

        let elapsed = self
            .start_time
            .map(|start| {
                let duration = start.elapsed();
                let secs = duration.as_secs();
                let hours = secs / 3600;
                let minutes = (secs % 3600) / 60;
                let seconds = secs % 60;
                format!(" | {:02}:{:02}:{:02}", hours, minutes, seconds)
            })
            .unwrap_or_default();

        let status_text = format!(
            " {}{}/{} | Theme: {} | [←/→] Navigate | [N] Notes {} | [Q] Quit{} ",
            filename_part,
            self.current_index + 1,
            self.total_slides(),
            self.theme_name,
            if self.show_notes { "✓" } else { "" },
            elapsed
        );

        let status = Paragraph::new(Line::from(vec![Span::styled(
            status_text,
            self.stylesheet.status_bar(),
        )]));

        frame.render_widget(status, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lantern_core::slide::{Block, TextSpan};

    fn create_test_slides() -> Vec<Slide> {
        vec![
            Slide::with_blocks(vec![Block::Heading {
                level: 1,
                spans: vec![TextSpan::plain("Slide 1")],
            }]),
            Slide::with_blocks(vec![Block::Heading {
                level: 1,
                spans: vec![TextSpan::plain("Slide 2")],
            }]),
            Slide::with_blocks(vec![Block::Heading {
                level: 1,
                spans: vec![TextSpan::plain("Slide 3")],
            }]),
        ]
    }

    #[test]
    fn viewer_creation() {
        let slides = create_test_slides();
        let viewer = SlideViewer::new(slides, ThemeColors::default());
        assert_eq!(viewer.total_slides(), 3);
        assert_eq!(viewer.current_index(), 0);
    }

    #[test]
    fn viewer_navigation_next() {
        let slides = create_test_slides();
        let mut viewer = SlideViewer::new(slides, ThemeColors::default());

        viewer.next();
        assert_eq!(viewer.current_index(), 1);

        viewer.next();
        assert_eq!(viewer.current_index(), 2);

        viewer.next();
        assert_eq!(viewer.current_index(), 2);
    }

    #[test]
    fn viewer_navigation_previous() {
        let slides = create_test_slides();
        let mut viewer = SlideViewer::new(slides, ThemeColors::default());

        viewer.jump_to(2);
        assert_eq!(viewer.current_index(), 2);

        viewer.previous();
        assert_eq!(viewer.current_index(), 1);

        viewer.previous();
        assert_eq!(viewer.current_index(), 0);

        viewer.previous();
        assert_eq!(viewer.current_index(), 0);
    }

    #[test]
    fn viewer_jump_to() {
        let slides = create_test_slides();
        let mut viewer = SlideViewer::new(slides, ThemeColors::default());

        viewer.jump_to(2);
        assert_eq!(viewer.current_index(), 2);

        viewer.jump_to(0);
        assert_eq!(viewer.current_index(), 0);

        viewer.jump_to(10);
        assert_eq!(viewer.current_index(), 0);
    }

    #[test]
    fn viewer_toggle_notes() {
        let slides = create_test_slides();
        let mut viewer = SlideViewer::new(slides, ThemeColors::default());

        assert!(!viewer.is_showing_notes());

        viewer.toggle_notes();
        assert!(viewer.is_showing_notes());

        viewer.toggle_notes();
        assert!(!viewer.is_showing_notes());
    }

    #[test]
    fn viewer_current_slide() {
        let slides = create_test_slides();
        let mut viewer = SlideViewer::new(slides, ThemeColors::default());

        assert!(viewer.current_slide().is_some());

        viewer.jump_to(1);
        let slide = viewer.current_slide().unwrap();
        assert_eq!(slide.blocks.len(), 1);
    }

    #[test]
    fn viewer_empty_slides() {
        let viewer = SlideViewer::new(Vec::new(), ThemeColors::default());
        assert_eq!(viewer.total_slides(), 0);
        assert!(viewer.current_slide().is_none());
    }

    #[test]
    fn viewer_with_context() {
        let slides = create_test_slides();
        let start_time = Instant::now();
        let viewer = SlideViewer::with_context(
            slides,
            ThemeColors::default(),
            Some("presentation.md".to_string()),
            "dark".to_string(),
            Some(start_time),
        );

        assert_eq!(viewer.filename, Some("presentation.md".to_string()));
        assert_eq!(viewer.theme_name, "dark");
        assert!(viewer.start_time.is_some());
    }

    #[test]
    fn viewer_with_context_none_values() {
        let slides = create_test_slides();
        let viewer = SlideViewer::with_context(slides, ThemeColors::default(), None, "default".to_string(), None);

        assert_eq!(viewer.filename, None);
        assert_eq!(viewer.theme_name, "default");
        assert_eq!(viewer.start_time, None);
    }

    #[test]
    fn viewer_default_constructor() {
        let slides = create_test_slides();
        let viewer = SlideViewer::new(slides, ThemeColors::default());

        assert_eq!(viewer.filename, None);
        assert_eq!(viewer.theme_name, "default");
        assert_eq!(viewer.start_time, None);
    }
}
