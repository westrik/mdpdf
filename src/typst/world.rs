use crate::config::MdpdfConfig;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use typst::diag::FileError;
use typst::foundations::Bytes;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{World, compile};

pub struct MdpdfWorld {
    config: MdpdfConfig,
    main_code: String,
    files: HashMap<String, Vec<u8>>,
    fonts: LazyHash<FontBook>,
    font_data: Vec<Vec<u8>>, // Keep font data alive
}

impl MdpdfWorld {
    pub fn new(config: MdpdfConfig, main_code: String, files: HashMap<String, Vec<u8>>) -> Self {
        // Create a font book and populate it with embedded fonts
        let mut font_book = FontBook::new();
        let mut font_data = Vec::new();

        // Load embedded fonts
        let embedded_fonts = Self::get_embedded_fonts();

        for font_bytes in embedded_fonts {
            if let Some(font) = Font::new(Bytes::new(font_bytes.clone()), 0) {
                // Get font info and add to book
                let info = font.info();
                font_book.push(info.clone());
                font_data.push(font_bytes);
            }
        }

        let fonts = LazyHash::new(font_book);

        Self {
            config,
            main_code,
            files,
            fonts,
            font_data,
        }
    }

    fn get_embedded_fonts() -> Vec<Vec<u8>> {
        vec![
            // Noto Emoji fonts (this is first so we don't use emoji that are available in other fonts)
            include_bytes!("../../fonts/noto-emoji/NotoEmoji-Regular.ttf").to_vec(),
            // Libertinus fonts
            include_bytes!("../../fonts/libertinus/LibertinusSerif-Regular.ttf").to_vec(),
            include_bytes!("../../fonts/libertinus/LibertinusSerif-Bold.ttf").to_vec(),
            include_bytes!("../../fonts/libertinus/LibertinusSerif-Italic.ttf").to_vec(),
            include_bytes!("../../fonts/libertinus/LibertinusSerif-BoldItalic.ttf").to_vec(),
            // include_bytes!("../../fonts/libertinus/LibertinusSerif-Semibold.ttf").to_vec(),
            // include_bytes!("../../fonts/libertinus/LibertinusSerif-SemiboldItalic.ttf").to_vec(),
            // DejaVu fonts
            include_bytes!("../../fonts/dejavu/DejaVuSansMono.ttf").to_vec(),
            include_bytes!("../../fonts/dejavu/DejaVuSansMono-Bold.ttf").to_vec(),
        ]
    }

    pub fn compile_to_pdf(&self) -> Result<Vec<u8>> {
        let warned = compile(self);
        let document = warned.output.map_err(|errors| {
            let error_messages: Vec<String> = errors
                .into_iter()
                .map(|error| format!("{error:?}"))
                .collect();
            anyhow::anyhow!("Typst compilation failed: {}", error_messages.join(", "))
        })?;

        // Generate PDF using Typst's library
        let pdf_options = typst_pdf::PdfOptions::default();
        let pdf_bytes = typst_pdf::pdf(&document, &pdf_options).map_err(|errors| {
            let error_messages: Vec<String> = errors
                .into_iter()
                .map(|error| format!("{error:?}"))
                .collect();
            anyhow::anyhow!("PDF generation failed: {}", error_messages.join(", "))
        })?;

        Ok(pdf_bytes)
    }

    pub fn create_document_template(&self) -> String {
        let mut template = String::new();
        template.push_str("#set page(");

        // Set page size
        if let Some(page_size) = &self.config.page_size {
            match page_size {
                crate::config::PageSize::A4 => template.push_str("\"a4\""),
                crate::config::PageSize::Letter => template.push_str("\"us-letter\""),
                crate::config::PageSize::Legal => template.push_str("\"us-legal\""),
                crate::config::PageSize::Custom { width, height } => {
                    template.push_str(&format!("({width}in, {height}in)"));
                }
            }
        } else {
            template.push_str("\"letter\"");
        }

        // Add margins if specified
        if let Some(margins) = &self.config.margins {
            template.push_str(&format!(
                ", margin: (top: {}in, right: {}in, bottom: {}in, left: {}in)",
                margins.top, margins.right, margins.bottom, margins.left
            ));
        }

        template.push_str(")\n");

        // Set default font family for body text (use Libertinus Serif as default)
        let body_font = self
            .config
            .font_family
            .as_deref()
            .unwrap_or("Libertinus Serif");
        template.push_str(&format!("#set text(font: \"{body_font}\")\n"));

        // Set font size if specified in config
        if let Some(font_size) = &self.config.font_size {
            template.push_str(&format!("#set text(size: {font_size}pt)\n"));
        }

        // Note: Code blocks will use the default font from the font book
        // The font book includes DejaVu Sans which works well for code

        // Other preamble settings:
        // links
        template.push_str("#show link: underline\n");

        // horizontal rules
        // template.push_str("#let hrule = line(length: 100%, stroke: 1pt + gray)\n");
        template.push_str("#let hrule = line(length: 100%)\n");
        // blockquotes
        template.push_str("#set quote(block: true)\n");
        template.push_str("#show quote.where(block: true): block.with(stroke: (left: 2pt + gray, rest: none), above: 1em, below: 1.2em)\n");

        // This would be nice to enable but it's pretty expensive to run
        // template.push_str("#show raw.where(block: true): block.with(stroke: 1pt + gray, inset: 1em, radius: 4pt, width: 100%)\n");

        // numbering formats for nested ordered lists
        template.push_str("#set enum(numbering: \"1.a.i.A.I.α.\")\n");

        // don't fail on missing references
        template.push_str(
            "#show ref: it => { if query(it.target).len() > 0 { it } else { it.target } }\n",
        );

        if let Some(header) = &self.config.header {
            template.push_str(&format!("#set page(header: [{header}])\n"));
        }
        if let Some(footer) = &self.config.footer {
            template.push_str(&format!("#set page(footer: [{footer}])\n"));
        }
        template
    }
}

impl World for MdpdfWorld {
    fn library(&self) -> &LazyHash<typst::Library> {
        static LIBRARY: once_cell::sync::Lazy<LazyHash<typst::Library>> =
            once_cell::sync::Lazy::new(|| LazyHash::new(typst::Library::default()));
        &LIBRARY
    }
    fn book(&self) -> &LazyHash<FontBook> {
        &self.fonts
    }
    fn main(&self) -> FileId {
        FileId::new(None, VirtualPath::new("main"))
    }
    fn source(&self, id: FileId) -> Result<Source, FileError> {
        let vpath = id.vpath().clone();
        let vpath_str = vpath.as_rootless_path().to_string_lossy();
        if vpath_str == "main" {
            Ok(Source::new(id, self.main_code.clone()))
        } else {
            Err(FileError::NotFound(PathBuf::from(vpath_str.as_ref())))
        }
    }
    fn file(&self, id: FileId) -> Result<Bytes, FileError> {
        let vpath = id.vpath().clone();
        let path = vpath.as_rootless_path().to_string_lossy();
        if let Some(data) = self.files.get(path.as_ref()) {
            Ok(Bytes::new(data.clone()))
        } else {
            Err(FileError::NotFound(PathBuf::from(path.as_ref())))
        }
    }
    fn font(&self, id: usize) -> Option<Font> {
        // Return font from our stored font data
        if id < self.font_data.len()
            && let Some(font) = Font::new(Bytes::new(self.font_data[id].clone()), 0)
        {
            return Some(font);
        }
        None
    }
    fn today(&self, _offset: Option<i64>) -> Option<typst::foundations::Datetime> {
        None
    }
}
