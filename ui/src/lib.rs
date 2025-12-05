pub mod app;
pub mod image;
pub mod layout;
pub mod renderer;
pub mod viewer;

pub use app::App;
pub use image::ImageManager;
pub use layout::SlideLayout;
pub use renderer::{ImageInfo, render_slide_content, render_slide_with_images};
pub use viewer::SlideViewer;

pub use lantern_core::{
    slide::{Block, Slide, TextSpan},
    theme::ThemeColors,
};
