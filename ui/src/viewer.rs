use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use slides_core::{slide::Slide, theme::ThemeColors};

use crate::renderer::render_slide_content;

/// Slide viewer state manager
///
/// Manages current slide index, navigation, and speaker notes visibility.
pub struct SlideViewer {
    slides: Vec<Slide>,
    current_index: usize,
    show_notes: bool,
    theme: ThemeColors,
}

impl SlideViewer {
    /// Create a new slide viewer with slides and theme
    pub fn new(slides: Vec<Slide>, theme: ThemeColors) -> Self {
        Self { slides, current_index: 0, show_notes: false, theme }
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

    /// Render the current slide to the frame
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if let Some(slide) = self.current_slide() {
            let content = render_slide_content(&slide.blocks, &self.theme);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(format!(" Slide {}/{} ", self.current_index + 1, self.total_slides()))
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

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
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" Speaker Notes ")
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

                let paragraph = Paragraph::new(notes.clone())
                    .block(block)
                    .wrap(Wrap { trim: false })
                    .style(Style::default().fg(Color::Gray));

                frame.render_widget(paragraph, area);
            }
        }
    }

    /// Render status bar with navigation info
    pub fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let status_text = format!(
            " {}/{} | [←/→] Navigate | [N] Notes {} | [Q] Quit ",
            self.current_index + 1,
            self.total_slides(),
            if self.show_notes { "✓" } else { "" }
        );

        let status = Paragraph::new(Line::from(vec![Span::styled(
            status_text,
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]));

        frame.render_widget(status, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slides_core::slide::{Block, TextSpan};

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
}
