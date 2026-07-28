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
    /// Raw Typst code appended after the generated document template.
    /// Later `#set`/`#show` rules override the built-in defaults.
    pub custom_preamble: Option<String>,
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
            custom_preamble: None,
        }
    }
}

impl MdpdfConfig {
    pub fn parse_dimension(value: &str) -> Result<f64, String> {
        let value = value.trim();
        let (number, divisor) = if let Some(value) = value.strip_suffix("in") {
            (value, 1.0)
        } else if let Some(value) = value.strip_suffix("mm") {
            (value, 25.4)
        } else if let Some(value) = value.strip_suffix("cm") {
            (value, 2.54)
        } else {
            (value, 1.0)
        };
        number
            .parse::<f64>()
            .map(|number| number / divisor)
            .map_err(|_| format!("Invalid dimension: {value}"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    Custom { width: f64, height: f64 },
}

impl PageSize {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "a4" => Ok(Self::A4),
            "letter" => Ok(Self::Letter),
            "legal" => Ok(Self::Legal),
            value if value.contains('x') => {
                let parts: Vec<_> = value.split('x').collect();
                if parts.len() != 2 {
                    return Err("Custom page size must be WIDTHxHEIGHT".to_string());
                }
                Ok(Self::Custom {
                    width: MdpdfConfig::parse_dimension(parts[0])?,
                    height: MdpdfConfig::parse_dimension(parts[1])?,
                })
            }
            value => Err(format!("Invalid page size '{value}'")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Margins {
    pub fn parse(value: &str) -> Result<Self, String> {
        let value = MdpdfConfig::parse_dimension(value)?;
        Ok(Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dimensions_and_page_sizes() {
        assert!((MdpdfConfig::parse_dimension("25.4mm").unwrap() - 1.0).abs() < 1e-9);
        assert!((MdpdfConfig::parse_dimension("2.54cm").unwrap() - 1.0).abs() < 1e-9);
        assert!(matches!(PageSize::parse("a4"), Ok(PageSize::A4)));
        assert!(matches!(
            PageSize::parse("8.5inx11in"),
            Ok(PageSize::Custom { .. })
        ));
        assert!(Margins::parse("1in").is_ok());
        assert!(PageSize::parse("not-a-size").is_err());
    }
}
