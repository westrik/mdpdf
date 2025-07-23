use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use image::ImageFormat;
use reqwest::Client;
use std::path::PathBuf;
use tokio::fs;
use url::Url;

pub struct ImageProcessor {
    client: Client,
    cache_dir: Option<PathBuf>,
}

impl ImageProcessor {
    pub fn new(cache_dir: Option<PathBuf>) -> Self {
        Self {
            client: Client::new(),
            cache_dir,
        }
    }

    pub async fn process_image_url(&self, url: &str) -> Result<Vec<u8>> {
        if url.starts_with("data:") {
            self.process_data_url(url).await
        } else if url.starts_with("http://") || url.starts_with("https://") {
            self.download_image(url).await
        } else {
            Err(anyhow::anyhow!("Unsupported image URL format: {}", url))
        }
    }

    async fn process_data_url(&self, data_url: &str) -> Result<Vec<u8>> {
        // Parse data URL format: data:[<mediatype>][;base64],<data>
        let parts: Vec<&str> = data_url.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid data URL format"));
        }

        let header = parts[0];
        let data = parts[1];

        // Check if it's base64 encoded
        if !header.contains(";base64") {
            return Err(anyhow::anyhow!(
                "Only base64 encoded data URLs are supported"
            ));
        }

        // Decode base64 data
        let decoded = BASE64
            .decode(data)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64: {}", e))?;

        Ok(decoded)
    }

    async fn download_image(&self, url: &str) -> Result<Vec<u8>> {
        // Check if we have a cached version
        if let Some(cache_path) = self.get_cache_path(url).await?
            && cache_path.exists()
        {
            return Ok(fs::read(&cache_path).await?);
        }

        // Download the image
        let response = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download image: {}",
                response.status()
            ));
        }

        let image_data = response.bytes().await?;

        // Cache the image if we have a cache directory
        if let Some(cache_path) = self.get_cache_path(url).await? {
            if let Some(parent) = cache_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(&cache_path, &image_data).await?;
        }

        Ok(image_data.to_vec())
    }

    async fn get_cache_path(&self, url: &str) -> Result<Option<PathBuf>> {
        if let Some(ref cache_dir) = self.cache_dir {
            let url_obj = Url::parse(url)?;
            let host = url_obj.host_str().unwrap_or("unknown");
            let path = url_obj.path();
            let filename = path.split('/').next_back().unwrap_or("image");

            // Create a safe filename
            let safe_filename = filename
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || c == '.' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect::<String>();

            let cache_path = cache_dir.join(host).join(safe_filename);

            Ok(Some(cache_path))
        } else {
            Ok(None)
        }
    }

    /// Detect if the given data represents an SVG image
    fn is_svg(&self, data: &[u8]) -> bool {
        // Check for SVG XML declaration or root element
        let content = String::from_utf8_lossy(data);
        let trimmed = content.trim();

        // Check for SVG XML declaration
        if trimmed.starts_with("<?xml") && trimmed.contains("<svg") {
            return true;
        }

        // Check for SVG root element (with or without XML declaration)
        if trimmed.starts_with("<svg") {
            return true;
        }

        // Check for data URLs that specify SVG MIME type
        if let Ok(content_str) = String::from_utf8(data.to_vec())
            && content_str.starts_with("data:image/svg+xml")
        {
            return true;
        }

        false
    }

    pub async fn convert_to_typst_format(
        &self,
        image_data: &[u8],
        alt_text: &str,
    ) -> Result<(String, Vec<u8>)> {
        // Check if this is an SVG image
        if self.is_svg(image_data) {
            // Generate a unique filename for this SVG
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            image_data.hash(&mut hasher);
            let hash = hasher.finish();
            let filename = format!("image_{hash:x}.svg");

            // Return the SVG data as-is with .svg extension
            let typst_syntax = format!("#image(\"{filename}\", alt: \"{alt_text}\")");
            return Ok((typst_syntax, image_data.to_vec()));
        }

        // Try to load the image and convert it to a format Typst can handle
        let img = match image::load_from_memory(image_data) {
            Ok(img) => img,
            Err(e) => {
                return Err(e.into());
            }
        };

        // Convert to PNG format for better compatibility
        let mut output = Vec::new();
        match img.write_to(&mut std::io::Cursor::new(&mut output), ImageFormat::Png) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.into());
            }
        }

        // Generate a unique filename for this image
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        image_data.hash(&mut hasher);
        let hash = hasher.finish();
        let filename = format!("image_{hash:x}.png");

        // Return both the Typst syntax and the converted PNG data
        let typst_syntax = format!("#image(\"{filename}\", alt: \"{alt_text}\")");
        Ok((typst_syntax, output))
    }
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self::new(None)
    }
}
