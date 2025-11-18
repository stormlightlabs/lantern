use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};

/// Layout manager for slide presentation
///
/// Calculates screen layout with main slide area, optional notes panel, status bar, and optional help line.
pub struct SlideLayout {
    show_notes: bool,
    show_help: bool,
}

impl SlideLayout {
    pub fn new(show_notes: bool) -> Self {
        Self { show_notes, show_help: false }
    }

    /// Panel margin (horizontal, vertical) around bordered panels
    const PANEL_MARGIN: Margin = Margin {
        horizontal: 2,
        vertical: 1,
    };

    /// Calculate layout areas for the slide viewer
    ///
    /// Returns (main_area, notes_area, status_area, help_area) where notes_area and help_area are None if hidden.
    pub fn calculate(&self, area: Rect) -> (Rect, Option<Rect>, Rect, Option<Rect>) {
        let status_height = if self.show_help { 2 } else { 1 };

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(status_height)])
            .split(area);

        let content_area = vertical_chunks[0];
        let bottom_area = vertical_chunks[1];

        let (status_area, help_area) = if self.show_help {
            let bottom_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(bottom_area);
            (bottom_chunks[0], Some(bottom_chunks[1]))
        } else {
            (bottom_area, None)
        };

        if self.show_notes {
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(content_area);

            let main_with_margin = horizontal_chunks[0].inner(Self::PANEL_MARGIN);
            let notes_with_margin = horizontal_chunks[1].inner(Self::PANEL_MARGIN);

            (main_with_margin, Some(notes_with_margin), status_area, help_area)
        } else {
            let content_with_margin = content_area.inner(Self::PANEL_MARGIN);
            (content_with_margin, None, status_area, help_area)
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

    /// Update help visibility
    pub fn set_show_help(&mut self, show: bool) {
        self.show_help = show;
    }

    /// Check if help is visible
    pub fn is_showing_help(&self) -> bool {
        self.show_help
    }
}

impl Default for SlideLayout {
    fn default() -> Self {
        Self { show_notes: false, show_help: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_without_notes() {
        let layout = SlideLayout::new(false);
        let area = Rect::new(0, 0, 100, 50);
        let (main, notes, status, help) = layout.calculate(area);

        assert!(notes.is_none());
        assert!(help.is_none());
        assert_eq!(status.height, 1);
        assert!(main.height > status.height);
    }

    #[test]
    fn layout_with_notes() {
        let layout = SlideLayout::new(true);
        let area = Rect::new(0, 0, 100, 50);
        let (main, notes, status, help) = layout.calculate(area);

        assert!(notes.is_some());
        assert!(help.is_none());
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
        let (main, _notes, status, _help) = layout.calculate(area);

        assert_eq!(status.height, 1);
        assert!(main.height >= 3);
    }

    #[test]
    fn layout_proportions_with_notes() {
        let layout = SlideLayout::new(true);
        let area = Rect::new(0, 0, 100, 50);
        let (main, notes, _status, _help) = layout.calculate(area);

        let notes_area = notes.unwrap();
        let main_percentage = (main.width as f32 / area.width as f32) * 100.0;
        let notes_percentage = (notes_area.width as f32 / area.width as f32) * 100.0;

        assert!(main_percentage >= 55.0 && main_percentage <= 65.0);
        assert!(notes_percentage >= 35.0 && notes_percentage <= 45.0);
    }

    #[test]
    fn layout_with_help() {
        let mut layout = SlideLayout::new(false);
        layout.set_show_help(true);
        let area = Rect::new(0, 0, 100, 50);
        let (main, notes, status, help) = layout.calculate(area);

        assert!(notes.is_none());
        assert!(help.is_some());
        assert_eq!(status.height, 1);
        assert_eq!(help.unwrap().height, 1);
        assert!(main.height > status.height);
    }

    #[test]
    fn layout_toggle_help() {
        let mut layout = SlideLayout::default();
        assert!(!layout.is_showing_help());

        layout.set_show_help(true);
        assert!(layout.is_showing_help());

        layout.set_show_help(false);
        assert!(!layout.is_showing_help());
    }
}
