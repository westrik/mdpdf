use html_parser::{Dom, Element, Node};
use std::collections::HashMap;

use crate::escape_text;
use crate::utils::images::ImageProcessor;
pub struct HtmlToTypstConverter {
    // Track nested structures
    in_code_block: bool,
    in_pre_block: bool,
    in_list: bool,
    list_type: Option<String>, // "ul" or "ol"
    list_level: usize,
    in_table: bool,
    table_headers: Vec<String>,
    table_rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    in_table_cell: bool,
    current_cell_content: String,
}

impl HtmlToTypstConverter {
    pub fn new() -> Self {
        Self {
            in_code_block: false,
            in_pre_block: false,
            in_list: false,
            list_type: None,
            list_level: 0,
            in_table: false,
            table_headers: Vec::new(),
            table_rows: Vec::new(),
            current_row: Vec::new(),
            in_table_cell: false,
            current_cell_content: String::new(),
        }
    }

    // test only
    #[allow(dead_code)]
    async fn convert_html_to_typst_no_images(&mut self, html: &str) -> String {
        // Parse the HTML
        let dom = Dom::parse(html).unwrap_or_else(|_| Dom::parse("").unwrap());

        // Convert to Typst without image processing
        self.process_nodes(&dom.children, None, None).await
    }

    pub async fn convert_html_to_typst(
        &mut self,
        html: &str,
        image_processor: &ImageProcessor,
        image_files: &mut HashMap<String, Vec<u8>>,
    ) -> String {
        // Parse the HTML
        let dom = Dom::parse(html).unwrap_or_else(|_| Dom::parse("").unwrap());

        // Convert to Typst with image processing
        self.process_nodes(&dom.children, Some(image_processor), Some(image_files))
            .await
    }

    async fn process_nodes(
        &mut self,
        nodes: &[Node],
        image_processor: Option<&ImageProcessor>,
        mut image_files: Option<&mut HashMap<String, Vec<u8>>>,
    ) -> String {
        let mut result = String::new();

        for node in nodes.iter() {
            let node_result = if image_processor.is_some() && image_files.is_some() {
                // We know both are Some, so we can safely unwrap
                let processor = image_processor.unwrap();
                let files = image_files.as_mut().unwrap();
                Box::pin(self.process_node(node, Some(processor), Some(files))).await
            } else {
                self.process_node_sync(node)
            };

            result.push_str(&node_result);
        }

        result
    }

    fn process_node_sync(&mut self, node: &Node) -> String {
        match node {
            Node::Element(element) => self.process_element_sync(element),
            Node::Text(text) => self.process_text(text),
            Node::Comment(_) => String::new(), // Ignore comments
        }
    }

    async fn process_node(
        &mut self,
        node: &Node,
        image_processor: Option<&ImageProcessor>,
        image_files: Option<&mut HashMap<String, Vec<u8>>>,
    ) -> String {
        match node {
            Node::Element(element) => {
                self.process_element(element, image_processor, image_files)
                    .await
            }
            Node::Text(text) => self.process_text(text),
            Node::Comment(_) => String::new(), // Ignore comments
        }
    }

    // TODO: refactor so we're not duplicating things across sync and async (lol ai)
    fn process_element_sync(&mut self, element: &Element) -> String {
        // For sync processing, we just return the raw image syntax without processing
        let tag_name = element.name.to_lowercase();
        let mut result = String::new();

        match tag_name.as_str() {
            // Headings
            "h1" => {
                result.push_str(&format!("\n= {}\n", self.get_element_text(element)));
            }
            "h2" => {
                result.push_str(&format!("\n== {}\n", self.get_element_text(element)));
            }
            "h3" => {
                result.push_str(&format!("\n=== {}\n", self.get_element_text(element)));
            }
            "h4" => {
                result.push_str(&format!("\n==== {}\n", self.get_element_text(element)));
            }
            "h5" => {
                result.push_str(&format!("\n===== {}\n", self.get_element_text(element)));
            }
            "h6" => {
                result.push_str(&format!("\n====== {}\n", self.get_element_text(element)));
            }

            // Paragraphs and breaks
            "p" => {
                if !self.in_code_block && !self.in_pre_block {
                    result.push('\n');
                }
                result.push_str(&self.process_children(element));
                if !self.in_code_block && !self.in_pre_block {
                    result.push('\n');
                }
            }
            "br" | "br/" => {
                result.push('\n');
            }

            // Horizontal rules
            "hr" | "hr/" => {
                result.push_str("\n#hrule\n");
            }

            // Text formatting
            "strong" | "b" => {
                result.push_str("#strong[");
                result.push_str(&self.process_children(element));
                result.push(']');
            }
            "em" | "i" => {
                result.push_str("#emph[");
                result.push_str(&self.process_children(element));
                result.push(']');
            }
            "strike" | "s" | "del" => {
                result.push_str("#strike[");
                result.push_str(&self.process_children(element));
                result.push(']');
            }
            "u" => {
                result.push_str("#underline[");
                result.push_str(&self.process_children(element));
                result.push(']');
            }

            // Code elements
            "code" => {
                if self.in_pre_block {
                    // Inside pre block, just add the text
                    result.push_str(&self.get_unescaped_element_text(element));
                } else {
                    // Inline code
                    result.push('`');
                    result.push_str(&self.get_unescaped_element_text(element));
                    result.push('`');
                }
            }
            "pre" => {
                self.in_pre_block = true;
                result.push_str("```\n");
                result.push_str(&self.process_children(element));
                result.push_str("\n```");
                self.in_pre_block = false;
            }

            // Links
            "a" => {
                if let Some(href) = element.attributes.get("href") {
                    if let Some(href_str) = href {
                        result.push_str(&format!("#link(\"{href_str}\")["));
                        result.push_str(&self.process_children(element));
                        result.push(']');
                    } else {
                        result.push_str(&self.process_children(element));
                    }
                } else {
                    result.push_str(&self.process_children(element));
                }
            }

            // Images - for sync processing, just output raw syntax
            "img" => {
                if let Some(src) = element.attributes.get("src")
                    && let Some(src_str) = src
                {
                    let alt = element
                        .attributes
                        .get("alt")
                        .and_then(|alt_opt| alt_opt.as_ref())
                        .map_or("", |v| v);
                    result.push_str(&format!(
                        "#image(\"{}\", alt: \"{}\")",
                        src_str,
                        escape_text(alt)
                    ));
                }
            }

            // Lists
            "ul" => {
                self.in_list = true;
                self.list_type = Some("ul".to_string());
                self.list_level += 1;
                result.push('\n');
                result.push_str(&self.process_children(element));
                self.list_level -= 1;
                if self.list_level == 0 {
                    self.in_list = false;
                    self.list_type = None;
                }
            }
            "ol" => {
                self.in_list = true;
                self.list_type = Some("ol".to_string());
                self.list_level += 1;
                result.push('\n');
                result.push_str(&self.process_children(element));
                self.list_level -= 1;
                if self.list_level == 0 {
                    self.in_list = false;
                    self.list_type = None;
                }
            }
            "li" => {
                let indentation = "  ".repeat(self.list_level.saturating_sub(1));
                match self.list_type.as_deref() {
                    Some("ol") => {
                        result.push_str(&format!("{indentation}+ "));
                    }
                    _ => {
                        result.push_str(&format!("{indentation}- "));
                    }
                }
                result.push_str(&self.process_children(element));
                result.push('\n');
            }

            // Blockquotes
            "blockquote" => {
                let content = self.process_children(element).trim().to_string();
                if !content.contains('\n') {
                    result.push_str(&format!("#quote[{content}]"));
                } else {
                    result.push_str("\n#quote[\n");
                    result.push_str(&content);
                    result.push_str("\n]\n");
                }
            }

            // Tables
            "table" => {
                self.in_table = true;
                self.table_headers.clear();
                self.table_rows.clear();
                result.push_str(&self.process_children(element));
                result.push_str(&self.finish_table());
                self.in_table = false;
            }
            "tr" => {
                self.current_row.clear();
                result.push_str(&self.process_children(element));
                if !self.current_row.is_empty() {
                    self.table_rows.push(self.current_row.clone());
                }
            }
            "th" => {
                self.in_table_cell = true;
                self.current_cell_content.clear();
                result.push_str(&self.process_children(element));
                self.table_headers.push(self.current_cell_content.clone());
                self.in_table_cell = false;
            }
            "td" => {
                self.in_table_cell = true;
                self.current_cell_content.clear();
                result.push_str(&self.process_children(element));
                self.current_row.push(self.current_cell_content.clone());
                self.in_table_cell = false;
            }

            // Definition lists
            "dl" => {
                result.push_str(&self.process_children(element));
            }
            "dt" => {
                result.push_str("\n/ ");
                result.push_str(&self.process_children(element));
                result.push(':');
            }
            "dd" => {
                result.push_str(&self.process_children(element));
                result.push('\n');
            }

            // Subscript and superscript
            "sub" => {
                result.push_str("#sub[");
                result.push_str(&self.process_children(element));
                result.push(']');
            }
            "sup" => {
                result.push_str("#super[");
                result.push_str(&self.process_children(element));
                result.push(']');
            }

            // Center alignment
            "center" => {
                result.push_str("#{set align(center);[");
                result.push_str(&self.process_children(element));
                result.push_str("]}");
            }

            // Generic containers (div, span, etc.)
            "div" | "span" | "section" | "article" | "header" | "footer" | "nav" | "aside" => {
                if element.name == "div" {
                    // div should add newlines like paragraphs
                    if !self.in_code_block && !self.in_pre_block {
                        result.push('\n');
                    }
                    result.push_str(&self.process_children(element));
                    if !self.in_code_block && !self.in_pre_block {
                        result.push('\n');
                    }
                } else {
                    // span and others just process children
                    result.push_str(&self.process_children(element));
                }
            }

            // Strip these elements completely
            "script" | "style" | "noscript" | "iframe" | "object" | "embed" | "video" | "audio"
            | "canvas" => {
                // Do nothing - strip these elements completely
            }

            // Convert to plain text for other unsupported elements
            _ => {
                result.push_str(&self.process_children(element));
            }
        }

        result
    }

    async fn process_element(
        &mut self,
        element: &Element,
        image_processor: Option<&ImageProcessor>,
        image_files: Option<&mut HashMap<String, Vec<u8>>>,
    ) -> String {
        let tag_name = element.name.to_lowercase();
        let mut result = String::new();

        match tag_name.as_str() {
            // Headings
            "h1" => {
                result.push_str(&format!("\n= {}\n", self.get_element_text(element)));
            }
            "h2" => {
                result.push_str(&format!("\n== {}\n", self.get_element_text(element)));
            }
            "h3" => {
                result.push_str(&format!("\n=== {}\n", self.get_element_text(element)));
            }
            "h4" => {
                result.push_str(&format!("\n==== {}\n", self.get_element_text(element)));
            }
            "h5" => {
                result.push_str(&format!("\n===== {}\n", self.get_element_text(element)));
            }
            "h6" => {
                result.push_str(&format!("\n====== {}\n", self.get_element_text(element)));
            }

            // Paragraphs and breaks
            "p" => {
                if !self.in_code_block && !self.in_pre_block {
                    result.push('\n');
                }
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                if !self.in_code_block && !self.in_pre_block {
                    result.push('\n');
                }
            }
            "br" | "br/" => {
                result.push('\n');
            }

            // Horizontal rules
            "hr" | "hr/" => {
                result.push_str("\n#hrule\n");
            }

            // Text formatting
            "strong" | "b" => {
                result.push_str("#strong[");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push(']');
            }
            "em" | "i" | "cite" => {
                result.push_str("#emph[");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push(']');
            }
            "strike" | "s" | "del" => {
                result.push_str("#strike[");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push(']');
            }
            "u" => {
                result.push_str("#underline[");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push(']');
            }
            "q" => {
                result.push('"');
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push('"');
            }
            // Code elements
            "code" => {
                if self.in_pre_block {
                    // Inside pre block, just add the text
                    result.push_str(&self.get_unescaped_element_text(element));
                } else {
                    // Inline code
                    result.push('`');
                    result.push_str(&self.get_unescaped_element_text(element));
                    result.push('`');
                }
            }
            "pre" => {
                self.in_pre_block = true;
                result.push_str("```\n");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push_str("\n```");
                self.in_pre_block = false;
            }

            // Links
            "a" => {
                if let Some(href) = element.attributes.get("href") {
                    if let Some(href_str) = href {
                        result.push_str(&format!("#link(\"{href_str}\")["));
                        result.push_str(
                            &self
                                .process_children_async(element, image_processor, image_files)
                                .await,
                        );
                        result.push(']');
                    } else {
                        result.push_str(
                            &self
                                .process_children_async(element, image_processor, image_files)
                                .await,
                        );
                    }
                } else {
                    result.push_str(
                        &self
                            .process_children_async(element, image_processor, image_files)
                            .await,
                    );
                }
            }

            // Images
            "img" => {
                if let Some(src) = element.attributes.get("src")
                    && let Some(src_str) = src
                {
                    let alt = element
                        .attributes
                        .get("alt")
                        .and_then(|alt_opt| alt_opt.as_ref())
                        .map_or("", |v| v);

                    if let (Some(processor), Some(files)) = (image_processor, image_files) {
                        // Process the image using the ImageProcessor
                        match processor.process_image_url(src_str).await {
                            Ok(image_data) => {
                                // Convert to Typst format and add to image files
                                match processor.convert_to_typst_format(&image_data, alt).await {
                                    Ok((typst_image_code, converted_png_data)) => {
                                        // Extract filename from the generated code
                                        if let Some(filename) = typst_image_code.split('"').nth(1) {
                                            let _png_data_len = converted_png_data.len();
                                            files.insert(filename.to_string(), converted_png_data);
                                            result.push_str(&typst_image_code);
                                        } else {
                                            // Fallback if parsing fails
                                            result.push_str(&format!(
                                                "#emph[Image: {}]",
                                                escape_text(alt)
                                            ));
                                        }
                                    }
                                    Err(_e) => {
                                        // Fallback on conversion error
                                        result.push_str(&format!(
                                            "#emph[Image: {}]",
                                            escape_text(alt)
                                        ));
                                    }
                                }
                            }
                            Err(_e) => {
                                // Fallback on processing error
                                result.push_str(&format!("#emph[Image: {}]", escape_text(alt)));
                            }
                        }
                    } else {
                        // No image processor, just output raw syntax
                        result.push_str(&format!(
                            "#image(\"{}\", alt: \"{}\")",
                            src_str,
                            escape_text(alt)
                        ));
                    }
                }
            }

            // Lists
            "ul" => {
                self.in_list = true;
                self.list_type = Some("ul".to_string());
                self.list_level += 1;
                result.push('\n');
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                self.list_level -= 1;
                if self.list_level == 0 {
                    self.in_list = false;
                    self.list_type = None;
                }
            }
            "ol" => {
                self.in_list = true;
                self.list_type = Some("ol".to_string());
                self.list_level += 1;
                result.push('\n');
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                self.list_level -= 1;
                if self.list_level == 0 {
                    self.in_list = false;
                    self.list_type = None;
                }
            }
            "li" => {
                let indentation = "  ".repeat(self.list_level.saturating_sub(1));
                match self.list_type.as_deref() {
                    Some("ol") => {
                        result.push_str(&format!("{indentation}+ "));
                    }
                    _ => {
                        result.push_str(&format!("{indentation}- "));
                    }
                }
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push('\n');
            }

            // Blockquotes
            "blockquote" => {
                let attribution = if let Some(cite) = element.attributes.get("cite") {
                    if let Some(cite_str) = cite {
                        format!("(attribution: \"{cite_str}\")")
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                let content = self
                    .process_children_async(element, image_processor, image_files)
                    .await
                    .trim()
                    .to_string();
                if !content.contains('\n') {
                    result.push_str(&format!("#quote{attribution}[{content}]"));
                } else {
                    result.push_str(&format!("\n#quote{attribution}[\n"));
                    result.push_str(&content);
                    result.push_str("\n]\n");
                }
            }

            // Tables
            "table" => {
                self.in_table = true;
                self.table_headers.clear();
                self.table_rows.clear();
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push_str(&self.finish_table());
                self.in_table = false;
            }
            "tr" => {
                self.current_row.clear();
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                if !self.current_row.is_empty() {
                    self.table_rows.push(self.current_row.clone());
                }
            }
            "th" => {
                self.in_table_cell = true;
                self.current_cell_content.clear();
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                self.table_headers.push(self.current_cell_content.clone());
                self.in_table_cell = false;
            }
            "td" => {
                self.in_table_cell = true;
                self.current_cell_content.clear();
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                self.current_row.push(self.current_cell_content.clone());
                self.in_table_cell = false;
            }

            // Definition lists
            "dl" => {
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
            }
            "dt" => {
                result.push_str("\n/ ");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push_str(": ");
            }
            "dd" => {
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push('\n');
            }

            // Subscript and superscript
            "sub" => {
                result.push_str("#sub[");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push(']');
            }
            "sup" => {
                result.push_str("#super[");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push(']');
            }

            // Center alignment
            "center" => {
                result.push_str("#{set align(center);[");
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
                result.push_str("]}");
            }

            // Container elements
            "div" | "span" | "section" | "article" | "header" | "footer" | "nav" | "aside" => {
                if element.name == "div" {
                    // div should add newlines like paragraphs
                    if !self.in_code_block && !self.in_pre_block {
                        result.push('\n');
                    }
                    result.push_str(
                        &self
                            .process_children_async(element, image_processor, image_files)
                            .await,
                    );
                    if !self.in_code_block && !self.in_pre_block {
                        result.push('\n');
                    }
                } else {
                    // span and others just process children
                    result.push_str(
                        &self
                            .process_children_async(element, image_processor, image_files)
                            .await,
                    );
                }
            }

            // Strip these elements completely
            "script" | "style" | "noscript" | "iframe" | "object" | "embed" | "video" | "audio"
            | "canvas" => {
                // Do nothing - strip these elements completely
            }

            // Convert to plain text for other unsupported elements
            _ => {
                result.push_str(
                    &self
                        .process_children_async(element, image_processor, image_files)
                        .await,
                );
            }
        }

        result
    }

    fn process_text(&mut self, text: &str) -> String {
        let decoded = escape_text(text);

        if self.in_table_cell {
            self.current_cell_content.push_str(&decoded);
            String::new()
        } else {
            decoded
        }
    }

    fn process_children(&mut self, element: &Element) -> String {
        // For sync processing, we need to handle nested elements properly
        let mut result = String::new();
        for child in &element.children {
            match child {
                Node::Element(child_element) => {
                    result.push_str(&self.process_element_sync(child_element));
                }
                Node::Text(text) => {
                    result.push_str(&self.process_text(text));
                }
                Node::Comment(_) => {
                    // Ignore comments
                }
            }
        }
        result
    }

    async fn process_children_async(
        &mut self,
        element: &Element,
        image_processor: Option<&ImageProcessor>,
        image_files: Option<&mut HashMap<String, Vec<u8>>>,
    ) -> String {
        Box::pin(self.process_nodes(&element.children, image_processor, image_files)).await
    }

    fn get_element_text(&mut self, element: &Element) -> String {
        let mut text = String::new();
        for child in &element.children {
            if let Node::Text(text_node) = child {
                text.push_str(&escape_text(text_node));
            }
        }
        text
    }

    fn get_unescaped_element_text(&mut self, element: &Element) -> String {
        let mut text = String::new();
        for child in &element.children {
            if let Node::Text(text_node) = child {
                // Apply Unicode filtering but don't escape Typst syntax
                let filtered = crate::filter_problematic_unicode(text_node);
                text.push_str(&filtered);
            }
        }
        text
    }

    fn finish_table(&mut self) -> String {
        if self.table_headers.is_empty() && self.table_rows.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        result.push_str("#{\n");
        result.push_str("  show table.cell.where(y: 0): strong\n");

        let column_count = if !self.table_headers.is_empty() {
            self.table_headers.len()
        } else if !self.table_rows.is_empty() {
            self.table_rows[0].len()
        } else {
            0
        };

        if column_count > 0 {
            result.push_str(&format!("  table(\n      columns: {column_count},\n"));

            // Add headers if present
            if !self.table_headers.is_empty() {
                result.push_str("      ");
                for (i, header) in self.table_headers.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&format!("[{header}]"));
                }
                result.push_str(",\n");
            }

            // Add data rows
            for row in &self.table_rows {
                result.push_str("      ");
                for (i, cell) in row.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&format!("[{cell}]"));
                }
                result.push_str(",\n");
            }

            result.push_str("  )\n");
        }

        result.push_str("}\n");
        result
    }
}

impl Default for HtmlToTypstConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_html() {
        let mut converter = HtmlToTypstConverter::new();
        let html = "<p>Hello <strong>world</strong></p>";
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("Hello"));
        assert!(result.contains("#strong[world]"));
    }

    #[tokio::test]
    async fn test_basic_formatting() {
        let mut converter = HtmlToTypstConverter::new();
        let html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("#strong[bold]"));
        assert!(result.contains("#emph[italic]"));
    }

    #[tokio::test]
    async fn test_headings() {
        let mut converter = HtmlToTypstConverter::new();
        let html = "<h1>Title</h1><h2>Subtitle</h2><h3>Section</h3>";
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("= Title"));
        assert!(result.contains("== Subtitle"));
        assert!(result.contains("=== Section"));
    }

    #[tokio::test]
    async fn test_links() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<a href="https://example.com">Link text</a>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains(r#"#link("https://example.com")[Link text]"#));
    }

    #[tokio::test]
    async fn test_images() {
        let mut converter = HtmlToTypstConverter::new();
        // let html = r#"<img src="image.jpg" alt="This is a description &quot;with quotes&quot;">"#;
        let html = r#"<img src="image.jpg" alt="This is a description with &quot;quotes&quot;">"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(
            result.contains(r#"#image("image.jpg", alt: "This is a description with \"quotes\"")"#)
        );
    }

    #[tokio::test]
    async fn test_lists() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<ul><li>Item 1</li><li>Item 2</li></ul>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("- Item 1"));
        assert!(result.contains("- Item 2"));
    }

    #[tokio::test]
    async fn test_code_blocks() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<pre><code>fn main() {
    println!("Hello");
}</code></pre>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains(
            r#"```
fn main() {
    println!("Hello");
}
```"#
        ));
    }

    #[tokio::test]
    async fn test_blockquotes() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<blockquote>This is a quote.</blockquote>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("#quote["));
        assert!(result.contains("This is a quote."));
    }

    #[tokio::test]
    async fn test_tables() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<table>
<tr><th>Header 1</th><th>Header 2</th></tr>
<tr><td>Cell 1</td><td>Cell 2</td></tr>
</table>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("table("));
        assert!(result.contains("columns: 2"));
        assert!(result.contains("[Header 1]"));
        assert!(result.contains("[Cell 1]"));
    }

    #[tokio::test]
    async fn test_html_entities() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<p>&amp; &lt; &gt; &quot; &apos;</p>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("& \\< \\> \\\" '"));
    }

    #[tokio::test]
    async fn test_nested_structures() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<div>
<p>Paragraph with <strong>bold <em>and italic</em> text</strong></p>
<ul>
<li>List item with <a href="https://example.com">link</a></li>
</ul>
</div>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("#strong[bold #emph[and italic] text]"));
        assert!(result.contains("#link(\"https://example.com\")[link]"));
    }

    #[tokio::test]
    async fn test_stripped_elements() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<p>Text</p><script>alert('hello');</script><style>body { color: red; }</style><p>More text</p>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("Text"));
        assert!(result.contains("More text"));
        assert!(!result.contains("alert"));
        assert!(!result.contains("color: red"));
    }

    #[tokio::test]
    async fn test_malformed_html() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<p>Unclosed paragraph<strong>Unclosed bold<em>Nested unclosed italic</em>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        // Should handle gracefully without crashing
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_video_tags() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<video>Video content</video>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        // Video tags should be stripped completely
        assert!(!result.contains("video"));
    }

    #[tokio::test]
    async fn test_video_tag_parsing() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<video></video>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        // Should be stripped completely
        assert!(!result.contains("video"));
    }

    #[tokio::test]
    async fn test_center_tag() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<center>This text should be centered</center>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains("#{set align(center);[This text should be centered]}"));
    }

    #[tokio::test]
    async fn test_center_tag_with_formatting() {
        let mut converter = HtmlToTypstConverter::new();
        let html =
            r#"<center>This text has <strong>bold</strong> and <em>italic</em> content</center>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        assert!(result.contains(
            "#{set align(center);[This text has #strong[bold] and #emph[italic] content]}"
        ));
    }

    #[tokio::test]
    async fn test_inline_code() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<p>Inline code: <code>println!("Hello, world!");</code></p>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        println!("XXX:{}", result);
        assert!(result.contains("Inline code: `println!(\"Hello, world!\");`"));
    }

    #[tokio::test]
    async fn test_fuzz_12() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"<U>xyz"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        println!("{}", result);
        assert_eq!(result, "\\<U\\>xyz");
    }

    #[tokio::test]
    async fn test_unsupported_html_tags() {
        let mut converter = HtmlToTypstConverter::new();
        let html = r#"
        <q>
        <bdi>
        <bdo>
        <ruby>
        <rt>
        <rp>"#;
        let result = converter.convert_html_to_typst_no_images(html).await;
        println!("{}", result);
        assert_eq!(
            result,
            "
        \\<q\\>
        \\<bdi\\>
        \\<bdo\\>
        \\<ruby\\>
        \\<rt\\>
        \\<rp\\>"
        );
    }
}
