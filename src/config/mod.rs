use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdpdfConfig {
    pub page_size: Option<PageSize>,
    pub margins: Option<Margins>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub image_handling: Option<ImageHandlingConfig>,
}

impl Default for MdpdfConfig {
    fn default() -> Self {
        Self {
            page_size: Some(PageSize::Letter),
            margins: Some(Margins::default()),
            font_family: Some("Libertinus Serif".to_string()),
            font_size: Some(13.0),
            header: None,
            footer: None,
            image_handling: Some(ImageHandlingConfig::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    Custom { width: f64, height: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 1.0,
            bottom: 1.0,
            left: 1.0,
            right: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageHandlingConfig {
    pub download_timeout: u64, // in milliseconds
    pub cache_directory: Option<String>,
    pub max_image_size: Option<u64>, // in bytes
    pub allowed_domains: Vec<String>,
}

impl Default for ImageHandlingConfig {
    fn default() -> Self {
        Self {
            download_timeout: 2000, // 2 seconds
            // cache_directory: Some(".mdpdf-cache".to_string()),
            cache_directory: None,
            max_image_size: Some(10 * 1024 * 1024), // 10MB
            allowed_domains: vec![],
        }
    }
}

impl ImageHandlingConfig {
    pub fn cache_directory_path(&self) -> Option<PathBuf> {
        self.cache_directory.as_ref().map(PathBuf::from)
    }
}
