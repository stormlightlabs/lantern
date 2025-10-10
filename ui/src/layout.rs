use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout manager for slide presentation
///
/// Calculates screen layout with main slide area, optional notes panel, and status bar.
pub struct SlideLayout {
    show_notes: bool,
}

impl SlideLayout {
    pub fn new(show_notes: bool) -> Self {
        Self { show_notes }
    }

    /// Calculate layout areas for the slide viewer
    ///
    /// Returns (main_area, notes_area, status_area) where notes_area is None if notes are hidden.
    pub fn calculate(&self, area: Rect) -> (Rect, Option<Rect>, Rect) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(1)])
            .split(area);

        let content_area = vertical_chunks[0];
        let status_area = vertical_chunks[1];

        if self.show_notes {
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(content_area);

            (horizontal_chunks[0], Some(horizontal_chunks[1]), status_area)
        } else {
            (content_area, None, status_area)
        }
    }

    /// Update notes visibility
    pub fn set_show_notes(&mut self, show: bool) {
        self.show_notes = show;
    }

    /// Check if notes are visible
    pub fn is_showing_notes(&self) -> bool {
        self.show_notes
    }
}

impl Default for SlideLayout {
    fn default() -> Self {
        Self { show_notes: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_without_notes() {
        let layout = SlideLayout::new(false);
        let area = Rect::new(0, 0, 100, 50);
        let (main, notes, status) = layout.calculate(area);

        assert!(notes.is_none());
        assert_eq!(status.height, 1);
        assert!(main.height > status.height);
    }

    #[test]
    fn layout_with_notes() {
        let layout = SlideLayout::new(true);
        let area = Rect::new(0, 0, 100, 50);
        let (main, notes, status) = layout.calculate(area);

        assert!(notes.is_some());
        let notes_area = notes.unwrap();
        assert!(main.width > notes_area.width);
        assert_eq!(main.height, notes_area.height);
        assert_eq!(status.height, 1);
    }

    #[test]
    fn layout_toggle_notes() {
        let mut layout = SlideLayout::default();
        assert!(!layout.is_showing_notes());

        layout.set_show_notes(true);
        assert!(layout.is_showing_notes());

        layout.set_show_notes(false);
        assert!(!layout.is_showing_notes());
    }

    #[test]
    fn layout_small_terminal() {
        let layout = SlideLayout::new(false);
        let area = Rect::new(0, 0, 20, 10);
        let (main, _notes, status) = layout.calculate(area);

        assert_eq!(status.height, 1);
        assert!(main.height >= 3);
    }

    #[test]
    fn layout_proportions_with_notes() {
        let layout = SlideLayout::new(true);
        let area = Rect::new(0, 0, 100, 50);
        let (main, notes, _status) = layout.calculate(area);

        let notes_area = notes.unwrap();
        let main_percentage = (main.width as f32 / area.width as f32) * 100.0;
        let notes_percentage = (notes_area.width as f32 / area.width as f32) * 100.0;

        assert!(main_percentage >= 55.0 && main_percentage <= 65.0);
        assert!(notes_percentage >= 35.0 && notes_percentage <= 45.0);
    }
}
