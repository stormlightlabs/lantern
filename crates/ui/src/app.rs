use lantern_core::{metadata::Meta, slide::Slide, term::InputEvent, theme::ThemeColors};
use ratatui::{
    Terminal as RatatuiTerminal,
    backend::Backend,
    style::{Color, Style},
    widgets::Block,
};
use std::io;
use std::time::{Duration, Instant};

use crate::{layout::SlideLayout, viewer::SlideViewer};

/// Main TUI application coordinator
///
/// Manages the presentation lifecycle, event loop, and component coordination.
pub struct App {
    viewer: SlideViewer,
    layout: SlideLayout,
    should_quit: bool,
    theme: ThemeColors,
    help_visible: bool,
}

impl App {
    /// Create a new presentation application
    pub fn new(slides: Vec<Slide>, theme: ThemeColors, filename: String, meta: Meta) -> Self {
        let viewer = SlideViewer::with_context(
            slides,
            theme,
            Some(filename.clone()),
            meta.theme.clone(),
            Some(Instant::now()),
        );

        Self { viewer, layout: SlideLayout::default(), should_quit: false, theme, help_visible: false }
    }

    /// Run the main event loop
    pub fn run<B: Backend>(&mut self, terminal: &mut RatatuiTerminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if self.should_quit {
                break;
            }

            if let Some(event) = InputEvent::poll(Duration::from_millis(50))? {
                self.handle_event(event);
            }
        }

        Ok(())
    }

    fn toggle_notes(&mut self) {
        self.viewer.toggle_notes();
        self.layout.set_show_notes(self.viewer.is_showing_notes())
    }

    fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
        self.layout.set_show_help(self.help_visible);
    }

    /// Handle input events
    fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::Next => self.viewer.next(),
            InputEvent::Previous => self.viewer.previous(),
            InputEvent::ToggleNotes => self.toggle_notes(),
            InputEvent::ToggleHelp => self.toggle_help(),
            InputEvent::Quit => self.should_quit = true,
            InputEvent::Resize { .. } | InputEvent::Search | InputEvent::Other => {}
        }
    }

    /// Draw the UI
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let bg_color = Color::Rgb(
            self.theme.ui_background.r,
            self.theme.ui_background.g,
            self.theme.ui_background.b,
        );

        let background = Block::default().style(Style::default().bg(bg_color));
        frame.render_widget(background, frame.area());

        let (main_area, notes_area, status_area, help_area) = self.layout.calculate(frame.area());

        self.viewer.render(frame, main_area);

        if let Some(notes_area) = notes_area {
            self.viewer.render_notes(frame, notes_area);
        }

        self.viewer.render_status_bar(frame, status_area);

        if let Some(help_area) = help_area {
            self.viewer.render_help_line(frame, help_area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lantern_core::slide::{Block, TextSpan};

    fn create_test_app() -> App {
        let slides = vec![
            Slide::with_blocks(vec![Block::Heading {
                level: 1,
                spans: vec![TextSpan::plain("Slide 1")],
            }]),
            Slide::with_blocks(vec![Block::Heading {
                level: 1,
                spans: vec![TextSpan::plain("Slide 2")],
            }]),
        ];

        App::new(slides, ThemeColors::default(), "test.md".to_string(), Meta::default())
    }

    #[test]
    fn app_creation() {
        let app = create_test_app();
        assert!(!app.should_quit);
    }

    #[test]
    fn app_handle_next() {
        let mut app = create_test_app();
        let initial_index = app.viewer.current_index();

        app.handle_event(InputEvent::Next);
        assert_eq!(app.viewer.current_index(), initial_index + 1);
    }

    #[test]
    fn app_handle_previous() {
        let mut app = create_test_app();
        app.handle_event(InputEvent::Next);
        app.handle_event(InputEvent::Previous);
        assert_eq!(app.viewer.current_index(), 0);
    }

    #[test]
    fn app_handle_toggle_notes() {
        let mut app = create_test_app();
        assert!(!app.viewer.is_showing_notes());

        app.handle_event(InputEvent::ToggleNotes);
        assert!(app.viewer.is_showing_notes());
        assert!(app.layout.is_showing_notes());
    }

    #[test]
    fn app_handle_quit() {
        let mut app = create_test_app();
        assert!(!app.should_quit);

        app.handle_event(InputEvent::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn app_handle_resize() {
        let mut app = create_test_app();
        app.handle_event(InputEvent::Resize { width: 100, height: 50 });
        assert!(!app.should_quit);
    }

    #[test]
    fn app_handle_toggle_help() {
        let mut app = create_test_app();
        assert!(!app.help_visible);
        assert!(!app.layout.is_showing_help());

        app.handle_event(InputEvent::ToggleHelp);
        assert!(app.help_visible);
        assert!(app.layout.is_showing_help());

        app.handle_event(InputEvent::ToggleHelp);
        assert!(!app.help_visible);
        assert!(!app.layout.is_showing_help());
    }
}
