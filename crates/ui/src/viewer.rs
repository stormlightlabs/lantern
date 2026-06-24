use lantern_core::{slide::Slide, theme::ThemeColors};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};
use ratatui_image::{Resize, StatefulImage};
use std::time::Instant;

use crate::image::ImageManager;
use crate::renderer::render_slide_with_images;

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
            .bg(Color::Rgb(
                self.theme.ui_border.r,
                self.theme.ui_border.g,
                self.theme.ui_border.b,
            ))
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
    image_manager: ImageManager,
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
            theme_name: "oxocarbon-dark".to_string(),
            start_time: None,
            image_manager: ImageManager::default(),
        }
    }

    /// Create a slide viewer with full presentation context
    pub fn with_context(
        slides: Vec<Slide>, theme: ThemeColors, filename: Option<String>, theme_name: String,
        start_time: Option<Instant>,
    ) -> Self {
        let mut image_manager = ImageManager::default();
        if let Some(ref path) = filename {
            image_manager.set_base_path(path);
        }

        Self {
            slides,
            current_index: 0,
            show_notes: false,
            stylesheet: theme.into(),
            filename,
            theme_name,
            start_time,
            image_manager,
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

    /// Jump to a specific slide by number (1-based)
    pub fn jump_to(&mut self, slide_number: usize) {
        if slide_number > 0 && slide_number <= self.slides.len() {
            self.current_index = slide_number - 1;
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

    /// Check if any slides have speaker notes
    pub fn has_notes(&self) -> bool {
        self.slides.iter().any(|slide| slide.notes.is_some())
    }

    /// Render the current slide to the frame
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if let Some(slide) = self.current_slide() {
            let (content, images) = render_slide_with_images(&slide.blocks, &self.theme());
            let border_color = self.stylesheet.border_color();
            let title_color = self.stylesheet.title_color();

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(format!(" Slide {}/{} ", self.current_index + 1, self.total_slides()))
                .title_style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
                .padding(Stylesheet::slide_padding());

            let inner_area = block.inner(area);
            frame.render_widget(block, area);

            let text_height = content.height() as u16;
            let mut text_content = Some(content);

            if !images.is_empty() {
                let total_images = images.len() as u16;
                let border_height_per_image = 1;
                let caption_height_per_image = 1;
                let min_image_content_height = 1;
                let min_height_per_image =
                    border_height_per_image + min_image_content_height + caption_height_per_image;
                let min_images_height = total_images * min_height_per_image;

                let available_height = inner_area.height;
                let max_text_height = available_height.saturating_sub(min_images_height);
                let text_area_height = text_height.min(max_text_height);

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(text_area_height), Constraint::Min(min_images_height)])
                    .split(inner_area);

                if chunks[0].height > 0 {
                    if let Some(text) = text_content.take() {
                        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
                        frame.render_widget(paragraph, chunks[0]);
                    }
                }

                let constraints: Vec<Constraint> = (0..total_images)
                    .map(|_| Constraint::Ratio(1, total_images as u32))
                    .collect();

                let image_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(chunks[1]);

                for (idx, img_info) in images.iter().enumerate() {
                    if let Ok(protocol) = self.image_manager.load_image(&img_info.path) {
                        let image_area = image_chunks[idx];

                        let horizontal_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Percentage(25),
                                Constraint::Percentage(50),
                                Constraint::Percentage(25),
                            ])
                            .split(image_area);

                        let centered_area = horizontal_chunks[1];

                        let image_block = Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(border_color));

                        let image_inner = image_block.inner(centered_area);
                        frame.render_widget(image_block, centered_area);

                        let caption_height = if img_info.alt.is_empty() { 0 } else { 1 };
                        let content_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Length(caption_height), Constraint::Min(1)])
                            .flex(Flex::Center)
                            .split(image_inner);

                        if caption_height > 0 {
                            let caption_style = Style::default()
                                .fg(Color::Rgb(150, 150, 150))
                                .add_modifier(Modifier::ITALIC);
                            let caption = Paragraph::new(Line::from(Span::styled(&img_info.alt, caption_style)))
                                .alignment(Alignment::Center);
                            frame.render_widget(caption, content_chunks[0]);
                        }

                        let resize = Resize::Fit(None);
                        let image_size = protocol.size_for(resize, content_chunks[1]);

                        let [centered_area] = Layout::horizontal([Constraint::Length(image_size.width)])
                            .flex(Flex::Center)
                            .areas(content_chunks[1]);
                        let [image_area] = Layout::vertical([Constraint::Length(image_size.height)])
                            .flex(Flex::Center)
                            .areas(centered_area);

                        let image_widget = StatefulImage::default();
                        frame.render_stateful_widget(image_widget, image_area, protocol);
                    }
                }
            } else if let Some(text) = text_content.take() {
                let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
                frame.render_widget(paragraph, inner_area);
            }
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
        let filename_part = self.filename.as_ref().map(|f| format!("{f} | ")).unwrap_or_default();

        let elapsed = self
            .start_time
            .map(|start| {
                let duration = start.elapsed();
                let secs = duration.as_secs();
                let hours = secs / 3600;
                let minutes = (secs % 3600) / 60;
                let seconds = secs % 60;
                format!(" | {hours:02}:{minutes:02}:{seconds:02}")
            })
            .unwrap_or_default();

        let notes_part = if self.has_notes() {
            format!(" | [N] Notes {}", if self.show_notes { "✓" } else { "" })
        } else {
            String::new()
        };

        let status_text = format!(
            " {}{}/{} | Theme: {}{}{} | [?] Help ",
            filename_part,
            self.current_index + 1,
            self.total_slides(),
            self.theme_name,
            notes_part,
            elapsed
        );

        let width = area.width as usize;
        let text_len = status_text.chars().count();
        let padding = if text_len < width { " ".repeat(width - text_len) } else { String::new() };

        let status = Paragraph::new(Line::from(vec![Span::styled(
            format!("{status_text}{padding}"),
            self.stylesheet.status_bar(),
        )]));

        frame.render_widget(status, area);
    }

    /// Render help line with keybinding reference
    pub fn render_help_line(&self, frame: &mut Frame, area: Rect) {
        let help_text = " [j/→/Space] Next | [k/←] Previous | [N] Toggle notes | [Q/Esc] Quit ";

        let width = area.width as usize;
        let text_len = help_text.chars().count();
        let padding = if text_len < width { " ".repeat(width - text_len) } else { String::new() };

        let full_text = format!("{help_text}{padding}");

        let dimmed_style = Style::default().fg(Color::Rgb(100, 100, 100)).bg(Color::Rgb(
            self.theme().ui_background.r,
            self.theme().ui_background.g,
            self.theme().ui_background.b,
        ));

        let help_line = Paragraph::new(Line::from(vec![Span::styled(full_text, dimmed_style)]));

        frame.render_widget(help_line, area);
    }

    fn theme(&self) -> ThemeColors {
        self.stylesheet.theme
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

        viewer.jump_to(3);
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

        viewer.jump_to(3);
        assert_eq!(viewer.current_index(), 2);

        viewer.jump_to(1);
        assert_eq!(viewer.current_index(), 0);

        viewer.jump_to(10);
        assert_eq!(viewer.current_index(), 0);

        viewer.jump_to(0);
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

        viewer.jump_to(2);
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
        let viewer =
            SlideViewer::with_context(slides, ThemeColors::default(), None, "oxocarbon-dark".to_string(), None);

        assert_eq!(viewer.filename, None);
        assert_eq!(viewer.theme_name, "oxocarbon-dark");
        assert_eq!(viewer.start_time, None);
    }

    #[test]
    fn viewer_default_constructor() {
        let slides = create_test_slides();
        let viewer = SlideViewer::new(slides, ThemeColors::default());

        assert_eq!(viewer.filename, None);
        assert_eq!(viewer.theme_name, "oxocarbon-dark");
        assert_eq!(viewer.start_time, None);
    }

    #[test]
    fn viewer_has_notes() {
        let slides_without_notes = create_test_slides();
        let viewer_no_notes = SlideViewer::new(slides_without_notes, ThemeColors::default());
        assert!(!viewer_no_notes.has_notes());

        let slides_with_notes = vec![Slide {
            blocks: vec![Block::Heading { level: 1, spans: vec![TextSpan::plain("Slide with notes")] }],
            notes: Some("These are speaker notes".to_string()),
        }];
        let viewer_with_notes = SlideViewer::new(slides_with_notes, ThemeColors::default());
        assert!(viewer_with_notes.has_notes());
    }
}
