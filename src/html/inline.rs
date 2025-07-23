use html_escape::decode_html_entities;
use std::collections::HashMap;

use crate::escape_text;
use crate::utils::images::ImageProcessor;

// HTML tag tracking structures
#[derive(Debug, Clone)]
pub struct OpenTag {
    tag_name: String,
    typst_close: String,
}

pub struct HtmlTagTracker {
    open_tags: Vec<OpenTag>,
}

impl HtmlTagTracker {
    pub fn new() -> Self {
        Self {
            open_tags: Vec::new(),
        }
    }

    pub fn open_tag(&mut self, tag_name: String, typst_close: String) {
        self.open_tags.push(OpenTag {
            tag_name,
            typst_close,
        });
    }

    pub fn close_tag(&mut self, tag_name: &str) -> Option<String> {
        // Find the most recent matching tag (LIFO order for proper nesting)
        if let Some(pos) = self
            .open_tags
            .iter()
            .rposition(|tag| tag.tag_name == tag_name)
        {
            let tag = self.open_tags.remove(pos);
            Some(tag.typst_close)
        } else {
            None
        }
    }

    pub fn close_all_tags(&mut self) -> String {
        let mut result = String::new();
        // Close tags in reverse order (LIFO)
        while let Some(tag) = self.open_tags.pop() {
            result.push_str(&tag.typst_close);
        }
        result
    }

    pub fn close_inline_tags(&mut self) -> String {
        // Close only inline formatting tags (not block-level tags)
        let mut result = String::new();
        let mut i = 0;
        while i < self.open_tags.len() {
            let tag = &self.open_tags[i];
            // Check if this is an inline tag that should be closed
            match tag.tag_name.as_str() {
                "u" | "strike" | "s" | "del" | "sup" | "sub" | "a" | "center" | "em" | "i"
                | "b" | "strong" | "code" | "q" | "cite" => {
                    let closed_tag = self.open_tags.remove(i);
                    result.push_str(&closed_tag.typst_close);
                    // Don't increment i since we removed an element
                }
                _ => {
                    // Keep block-level tags open
                    i += 1;
                }
            }
        }
        result
    }

    pub async fn process_html_tag(
        &mut self,
        html: &str,
        image_processor: &ImageProcessor,
        image_files: &mut HashMap<String, Vec<u8>>,
    ) -> String {
        let html = html.trim();

        // Handle opening tags
        if html.starts_with('<') && html.ends_with('>') && !html.starts_with("</") {
            let tag_content = &html[1..html.len() - 1]; // Remove < and >
            let parts: Vec<&str> = tag_content.split_whitespace().collect();

            if parts.is_empty() {
                return String::new();
            }

            let tag_name = parts[0].to_lowercase();

            // Extract attributes with proper quote handling
            let mut attributes = HashMap::new();
            let mut current_part = String::new();
            let mut in_quotes = false;
            let mut quote_char = '\0';
            let mut attr_parts = Vec::new();

            // Reconstruct the attribute string properly
            for part in parts.iter().skip(1) {
                if !in_quotes {
                    // Check if this part starts a quoted attribute
                    if part.contains('=') && !part.ends_with('"') && !part.ends_with('\'') {
                        // This might be the start of a quoted attribute
                        current_part = part.to_string();
                        if part.contains('"') {
                            in_quotes = true;
                            quote_char = '"';
                        } else if part.contains('\'') {
                            in_quotes = true;
                            quote_char = '\'';
                        } else {
                            // No quotes, this is a simple attribute
                            attr_parts.push(part.to_string());
                        }
                    } else {
                        // Simple attribute or complete quoted attribute
                        attr_parts.push(part.to_string());
                    }
                } else {
                    // We're inside quotes, accumulate until we find the closing quote
                    current_part.push(' ');
                    current_part.push_str(part);
                    if part.contains(quote_char) {
                        // Found closing quote
                        attr_parts.push(current_part.clone());
                        current_part.clear();
                        in_quotes = false;
                        quote_char = '\0';
                    }
                }
            }

            // If we're still in quotes, add the accumulated part
            if in_quotes {
                attr_parts.push(current_part);
            }

            // Parse the attributes
            for attr_part in attr_parts {
                if attr_part.contains('=') {
                    let equal_pos = attr_part.find('=').unwrap();
                    let key = &attr_part[..equal_pos];
                    let value_part = &attr_part[equal_pos + 1..];

                    // Handle quoted and unquoted values
                    let value = if value_part.starts_with('"') && value_part.ends_with('"') {
                        &value_part[1..value_part.len() - 1]
                    } else if value_part.starts_with('\'') && value_part.ends_with('\'') {
                        &value_part[1..value_part.len() - 1]
                    } else {
                        value_part
                    };

                    // Decode HTML entities in the value
                    let decoded_value = decode_html_entities(value).into_owned();
                    attributes.insert(key.to_string(), decoded_value);
                }
            }

            // Handle different tag types
            match tag_name.as_str() {
                "div" => "\n\n".to_string(),
                "p" => "\n\n".to_string(),
                "br" | "br/" => "\n".to_string(),
                "hr" | "hr/" => "\n#hrule\n".to_string(),
                "strong" | "b" => {
                    let typst_open = "#strong[".to_string();
                    let typst_close = "]".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "em" | "i" | "cite" => {
                    let typst_open = "#emph[".to_string();
                    let typst_close = "]".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "q" => {
                    let typst_open = "\"".to_string();
                    let typst_close = "\"".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "strike" | "s" | "del" => {
                    let typst_open = "#strike[".to_string();
                    let typst_close = "]".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "u" => {
                    let typst_open = "#underline[".to_string();
                    let typst_close = "]".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "code" => "`".to_string(),
                "a" => {
                    if let Some(href) = attributes.get("href") {
                        let typst_open = format!("#link(\"{href}\")[");
                        let typst_close = "]".to_string();
                        self.open_tag(tag_name, typst_close);
                        typst_open
                    } else {
                        String::new()
                    }
                }
                "img" => {
                    if let Some(src) = attributes.get("src") {
                        let empty_alt = String::new();
                        let alt = attributes.get("alt").unwrap_or(&empty_alt);

                        // Process the image using the ImageProcessor
                        match image_processor.process_image_url(src).await {
                            Ok(image_data) => {
                                // Convert to Typst format and add to image files
                                match image_processor
                                    .convert_to_typst_format(&image_data, alt)
                                    .await
                                {
                                    Ok((typst_image_code, converted_png_data)) => {
                                        // Extract filename from the generated code
                                        if let Some(filename) = typst_image_code.split('"').nth(1) {
                                            let _png_data_len = converted_png_data.len();
                                            image_files
                                                .insert(filename.to_string(), converted_png_data);
                                            typst_image_code
                                        } else {
                                            // Fallback if parsing fails
                                            format!("#emph[Image: {}]", escape_text(alt))
                                        }
                                    }
                                    Err(_e) => {
                                        // Fallback on conversion error
                                        format!("#emph[Image: {}]", escape_text(alt))
                                    }
                                }
                            }
                            Err(_e) => {
                                // Fallback on processing error
                                format!("#emph[Image: {}]", escape_text(alt))
                            }
                        }
                    } else {
                        String::new()
                    }
                }
                "h1" => "\n= ".to_string(),
                "h2" => "\n== ".to_string(),
                "h3" => "\n=== ".to_string(),
                "h4" => "\n==== ".to_string(),
                "h5" => "\n===== ".to_string(),
                "h6" => "\n====== ".to_string(),
                "ul" => String::new(),
                "ol" => String::new(),
                "li" => "\n- ".to_string(),
                "blockquote" => "\n#quote[\n".to_string(),
                "table" => "\n#{\n  table(\n".to_string(),
                "tr" => "    ".to_string(),
                "th" | "td" => "[".to_string(),
                "sup" => {
                    let typst_open = "#super[".to_string();
                    let typst_close = "]".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "sub" => {
                    let typst_open = "#sub[".to_string();
                    let typst_close = "]".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "center" => {
                    let typst_open = "#{set align(center);[".to_string();
                    let typst_close = "]}".to_string();
                    self.open_tag(tag_name, typst_close);
                    typst_open
                }
                "video" | "audio" | "canvas" | "script" | "style" => String::new(), // Strip these tags
                _ => String::new(),
            }
        }
        // Handle closing tags
        else if html.starts_with("</") && html.ends_with('>') {
            let tag_name = &html[2..html.len() - 1].to_lowercase();

            match tag_name.as_str() {
                "div" => "\n\n".to_string(),
                "p" => "\n\n".to_string(),
                "strong" | "b" => "]".to_string(),
                "em" | "i" | "cite" => self.close_tag(tag_name).unwrap_or_default(),
                "strike" | "s" | "del" | "u" | "sup" | "sub" | "center" | "a" => {
                    self.close_tag(tag_name).unwrap_or_default()
                }
                "q" => self.close_tag(tag_name).unwrap_or_default(),
                "code" => "`".to_string(),
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => "\n".to_string(),
                "ul" | "ol" => String::new(),
                "li" => "\n".to_string(),
                "blockquote" => "\n]\n".to_string(),
                "table" => "  )\n}\n".to_string(),
                "tr" => "\n".to_string(),
                "th" | "td" => "], ".to_string(),
                _ => String::new(),
            }
        }
        // Handle self-closing tags
        else if html.starts_with('<') && html.ends_with("/>") {
            let tag_content = &html[1..html.len() - 2]; // Remove < and />
            let parts: Vec<&str> = tag_content.split_whitespace().collect();
            if parts.is_empty() {
                return String::new();
            }

            let tag_name = parts[0].to_lowercase();

            // Extract attributes with proper quote handling (same logic as above)
            let mut attributes = HashMap::new();
            let mut current_part = String::new();
            let mut in_quotes = false;
            let mut quote_char = '\0';
            let mut attr_parts = Vec::new();

            // Reconstruct the attribute string properly
            for part in parts.iter().skip(1) {
                if !in_quotes {
                    // Check if this part starts a quoted attribute
                    if part.contains('=') && !part.ends_with('"') && !part.ends_with('\'') {
                        // This might be the start of a quoted attribute
                        current_part = part.to_string();
                        if part.contains('"') {
                            in_quotes = true;
                            quote_char = '"';
                        } else if part.contains('\'') {
                            in_quotes = true;
                            quote_char = '\'';
                        } else {
                            // No quotes, this is a simple attribute
                            attr_parts.push(part.to_string());
                        }
                    } else {
                        // Simple attribute or complete quoted attribute
                        attr_parts.push(part.to_string());
                    }
                } else {
                    // We're inside quotes, accumulate until we find the closing quote
                    current_part.push(' ');
                    current_part.push_str(part);
                    if part.contains(quote_char) {
                        // Found closing quote
                        attr_parts.push(current_part.clone());
                        current_part.clear();
                        in_quotes = false;
                        quote_char = '\0';
                    }
                }
            }

            // If we're still in quotes, add the accumulated part
            if in_quotes {
                attr_parts.push(current_part);
            }

            // Parse the attributes
            for attr_part in attr_parts {
                if attr_part.contains('=') {
                    let equal_pos = attr_part.find('=').unwrap();
                    let key = &attr_part[..equal_pos];
                    let value_part = &attr_part[equal_pos + 1..];

                    // Handle quoted and unquoted values
                    let value = if value_part.starts_with('"') && value_part.ends_with('"') {
                        &value_part[1..value_part.len() - 1]
                    } else if value_part.starts_with('\'') && value_part.ends_with('\'') {
                        &value_part[1..value_part.len() - 1]
                    } else {
                        value_part
                    };

                    // Decode HTML entities in the value
                    let decoded_value = decode_html_entities(value).into_owned();
                    attributes.insert(key.to_string(), decoded_value);
                }
            }

            match tag_name.as_str() {
                "br" => "\n".to_string(),
                "hr" => "\n#hrule\n".to_string(),
                "img" => {
                    if let Some(src) = attributes.get("src") {
                        let empty_alt = String::new();
                        let alt = attributes.get("alt").unwrap_or(&empty_alt);

                        // Process the image using the ImageProcessor
                        match image_processor.process_image_url(src).await {
                            Ok(image_data) => {
                                // Convert to Typst format and add to image files
                                match image_processor
                                    .convert_to_typst_format(&image_data, alt)
                                    .await
                                {
                                    Ok((typst_image_code, converted_png_data)) => {
                                        // Extract filename from the generated code
                                        if let Some(filename) = typst_image_code.split('"').nth(1) {
                                            let _png_data_len = converted_png_data.len();
                                            image_files
                                                .insert(filename.to_string(), converted_png_data);
                                            typst_image_code
                                        } else {
                                            // Fallback if parsing fails
                                            format!("#emph[Image: {}]", escape_text(alt))
                                        }
                                    }
                                    Err(_e) => {
                                        // Fallback on conversion error
                                        format!("#emph[Image: {}]", escape_text(alt))
                                    }
                                }
                            }
                            Err(_e) => {
                                // Fallback on processing error
                                format!("#emph[Image: {}]", escape_text(alt))
                            }
                        }
                    } else {
                        String::new()
                    }
                }
                _ => String::new(),
            }
        } else {
            String::new()
        }
    }
}
