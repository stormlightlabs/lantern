use image::DynamicImage;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

/// Manages image loading and protocol state for terminal rendering
///
/// Handles image loading from paths, protocol detection, and caching of loaded images.
pub struct ImageManager {
    picker: Picker,
    protocols: HashMap<String, StatefulProtocol>,
    base_path: Option<PathBuf>,
}

impl ImageManager {
    /// Create a new ImageManager with protocol detection
    pub fn new() -> io::Result<Self> {
        let picker = Picker::from_query_stdio().map_err(io::Error::other)?;

        Ok(Self { picker, protocols: HashMap::new(), base_path: None })
    }

    /// Set the base path for resolving relative image paths
    pub fn set_base_path(&mut self, path: impl AsRef<Path>) {
        self.base_path = Some(path.as_ref().to_path_buf());
    }

    /// Load an image from a path and create a protocol for it
    ///
    /// Returns a reference to the protocol if successful.
    pub fn load_image(&mut self, path: &str) -> io::Result<&mut StatefulProtocol> {
        if !self.protocols.contains_key(path) {
            let image_path = self.resolve_path(path);
            let dyn_img = load_image_from_path(&image_path)?;
            let protocol = self.picker.new_resize_protocol(dyn_img);
            self.protocols.insert(path.to_string(), protocol);
        }

        Ok(self.protocols.get_mut(path).unwrap())
    }

    /// Check if an image is already loaded
    pub fn has_image(&self, path: &str) -> bool {
        self.protocols.contains_key(path)
    }

    /// Get a mutable reference to a loaded image protocol
    pub fn get_protocol_mut(&mut self, path: &str) -> Option<&mut StatefulProtocol> {
        self.protocols.get_mut(path)
    }

    /// Resolve a path relative to the base path if set
    fn resolve_path(&self, path: &str) -> PathBuf {
        let path = Path::new(path);

        if path.is_absolute() {
            return path.to_path_buf();
        }

        if let Some(base) = &self.base_path {
            if let Some(parent) = base.parent() {
                return parent.join(path);
            }
        }

        path.to_path_buf()
    }
}

impl Default for ImageManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            picker: Picker::from_fontsize((8, 16)),
            protocols: HashMap::new(),
            base_path: None,
        })
    }
}

/// Load an image from a file path
fn load_image_from_path(path: &Path) -> io::Result<DynamicImage> {
    image::ImageReader::open(path)
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?
        .decode()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_path_absolute() {
        let mut manager = ImageManager::default();
        manager.set_base_path("/home/user/slides.md");
        let resolved = manager.resolve_path("/tmp/image.png");
        assert_eq!(resolved, PathBuf::from("/tmp/image.png"));
    }

    #[test]
    fn resolve_path_relative() {
        let mut manager = ImageManager::default();
        manager.set_base_path("/home/user/slides.md");
        let resolved = manager.resolve_path("images/test.png");
        assert_eq!(resolved, PathBuf::from("/home/user/images/test.png"));
    }

    #[test]
    fn resolve_path_no_base() {
        let manager = ImageManager::default();
        let resolved = manager.resolve_path("test.png");
        assert_eq!(resolved, PathBuf::from("test.png"));
    }

    #[test]
    fn has_image_returns_false_for_unloaded() {
        let manager = ImageManager::default();
        assert!(!manager.has_image("test.png"));
    }
}
