pub mod layout;
pub mod renderer;
pub mod viewer;

pub use layout::SlideLayout;
pub use renderer::render_slide_content;
pub use viewer::SlideViewer;

pub use slides_core::{
    slide::{Block, Slide, TextSpan},
    theme::ThemeColors,
};
