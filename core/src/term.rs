use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{io, time::Duration};

#[cfg(not(test))]
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

/// Terminal manager that handles setup and cleanup
///
/// Configures the terminal for TUI mode with alternate screen and raw mode.
/// Automatically restores terminal state on drop to prevent terminal corruption.
pub struct Terminal {
    in_alternate_screen: bool,
    in_raw_mode: bool,
}

impl Default for Terminal {
    fn default() -> Self {
        Self { in_alternate_screen: true, in_raw_mode: true }
    }
}

impl Terminal {
    /// Initialize terminal for TUI mode
    ///
    /// Enables alternate screen and raw mode for full terminal control.
    pub fn setup() -> io::Result<Self> {
        #[cfg(not(test))]
        {
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen)?;
            enable_raw_mode()?;
        }

        Ok(Self::default())
    }

    /// Restore terminal to normal mode by disabling raw mode and exits alternate screen.
    ///
    /// Called automatically on drop, but can be called manually for explicit cleanup.
    pub fn restore(&mut self) -> io::Result<()> {
        #[cfg(not(test))]
        {
            if self.in_raw_mode {
                disable_raw_mode()?;
                self.in_raw_mode = false;
            }

            if self.in_alternate_screen {
                let mut stdout = io::stdout();
                execute!(stdout, LeaveAlternateScreen)?;
                self.in_alternate_screen = false;
            }
        }

        #[cfg(test)]
        {
            self.in_raw_mode = false;
            self.in_alternate_screen = false;
        }

        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

/// Input event handler for slide navigation and control
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    /// Move to next slide
    Next,
    /// Move to previous slide
    Previous,
    /// Toggle speaker notes
    ToggleNotes,
    /// Toggle help display
    ToggleHelp,
    /// Search slides
    /// TODO: Implement search functionality
    Search,
    /// Quit presentation
    Quit,
    /// Terminal was resized
    /// NOTE: Terminal resize is handled automatically by ratatui
    Resize { width: u16, height: u16 },
    /// Unknown/unhandled event
    Other,
}

impl InputEvent {
    /// Convert crossterm event to input event
    ///
    /// Maps keyboard and terminal events to presentation actions.
    pub fn from_crossterm(event: Event) -> Self {
        match event {
            Event::Key(KeyEvent { code, modifiers, .. }) => Self::from_key(code, modifiers),
            Event::Resize(width, height) => Self::Resize { width, height },
            _ => Self::Other,
        }
    }

    /// Map key press to input event
    fn from_key(code: KeyCode, modifiers: KeyModifiers) -> Self {
        match (code, modifiers) {
            (KeyCode::Right | KeyCode::Char('j') | KeyCode::Char(' '), _) => Self::Next,
            (KeyCode::Char('n'), KeyModifiers::NONE) => Self::Next,
            (KeyCode::Left | KeyCode::Char('k'), _) => Self::Previous,
            (KeyCode::Char('p'), KeyModifiers::NONE) => Self::Previous,
            (KeyCode::Char('q'), KeyModifiers::NONE) => Self::Quit,
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => Self::Quit,
            (KeyCode::Esc, _) => Self::Quit,
            (KeyCode::Char('n'), KeyModifiers::SHIFT) => Self::ToggleNotes,
            (KeyCode::Char('?'), _) => Self::ToggleHelp,
            (KeyCode::Char('f'), KeyModifiers::CONTROL) => Self::Search,
            (KeyCode::Char('/'), KeyModifiers::NONE) => Self::Search,
            _ => Self::Other,
        }
    }

    /// Poll for next input event with timeout
    pub fn poll(timeout: Duration) -> io::Result<Option<Self>> {
        if event::poll(timeout)? {
            let event = event::read()?;
            Ok(Some(Self::from_crossterm(event)))
        } else {
            Ok(None)
        }
    }

    /// Read next input event (blocking until an event is available)
    pub fn read() -> io::Result<Self> {
        let event = event::read()?;
        Ok(Self::from_crossterm(event))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_event_navigation() {
        let next = InputEvent::from_key(KeyCode::Right, KeyModifiers::NONE);
        assert_eq!(next, InputEvent::Next);

        let prev = InputEvent::from_key(KeyCode::Left, KeyModifiers::NONE);
        assert_eq!(prev, InputEvent::Previous);
    }

    #[test]
    fn input_event_quit() {
        let quit_q = InputEvent::from_key(KeyCode::Char('q'), KeyModifiers::NONE);
        assert_eq!(quit_q, InputEvent::Quit);

        let quit_ctrl_c = InputEvent::from_key(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(quit_ctrl_c, InputEvent::Quit);
    }

    #[test]
    fn input_event_search() {
        let search_slash = InputEvent::from_key(KeyCode::Char('/'), KeyModifiers::NONE);
        assert_eq!(search_slash, InputEvent::Search);

        let search_ctrl_f = InputEvent::from_key(KeyCode::Char('f'), KeyModifiers::CONTROL);
        assert_eq!(search_ctrl_f, InputEvent::Search);
    }

    #[test]
    fn input_event_resize() {
        let resize = InputEvent::from_crossterm(Event::Resize(80, 24));
        assert_eq!(resize, InputEvent::Resize { width: 80, height: 24 });
    }

    #[test]
    fn input_event_toggle_help() {
        let help = InputEvent::from_key(KeyCode::Char('?'), KeyModifiers::NONE);
        assert_eq!(help, InputEvent::ToggleHelp);

        let help_shift = InputEvent::from_key(KeyCode::Char('?'), KeyModifiers::SHIFT);
        assert_eq!(help_shift, InputEvent::ToggleHelp);
    }

    #[test]
    fn terminal_default_state() {
        let terminal = Terminal::default();
        assert!(terminal.in_alternate_screen);
        assert!(terminal.in_raw_mode);
    }

    #[test]
    fn terminal_restore_idempotent() {
        let mut terminal = Terminal { in_alternate_screen: false, in_raw_mode: false };

        assert!(terminal.restore().is_ok());
        assert!(terminal.restore().is_ok());
        assert!(!terminal.in_alternate_screen);
        assert!(!terminal.in_raw_mode);
    }

    #[test]
    fn terminal_restore_clears_flags() {
        let mut terminal = Terminal { in_alternate_screen: false, in_raw_mode: false };
        let _ = terminal.restore();
        assert!(!terminal.in_alternate_screen);
        assert!(!terminal.in_raw_mode);
    }
}
