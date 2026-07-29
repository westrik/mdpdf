use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;

use mdpdf::{config::MdpdfConfig, markdown_to_typst, typst_to_pdf};

#[derive(Parser)]
#[command(
    name = "mdpdf",
    about = "Convert Markdown to PDF using Typst",
    version,
    long_about = "A fast Markdown to PDF converter that uses Typst for high-quality output."
)]
struct Args {
    /// Input Markdown file (use - for stdin)
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Output PDF file (defaults to stdout)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Page size (letter, legal, a4, or custom like "8.5inx11in")
    #[arg(long, value_name = "SIZE", default_value = "letter")]
    page_size: String,

    /// Margin size (e.g., "1in", "20mm")
    #[arg(long, value_name = "SIZE", default_value = "1in")]
    margin: String,

    /// Font size in points
    #[arg(long, value_name = "POINTS", default_value = "13")]
    font_size: f64,

    /// Add a table of contents at the beginning of the document
    #[arg(long)]
    toc: bool,

    /// Additional font file or directory (repeatable)
    #[arg(short = 'f', long = "font-path", value_name = "PATH")]
    font_paths: Vec<PathBuf>,
}

fn read_input(input: Option<PathBuf>) -> Result<String, String> {
    match input {
        None => {
            // Read from stdin
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .map_err(|e| format!("Failed to read from stdin: {}", e))?;
            Ok(content)
        }
        Some(path) => {
            if path.to_string_lossy() == "-" {
                // Read from stdin
                let mut content = String::new();
                io::stdin()
                    .read_to_string(&mut content)
                    .map_err(|e| format!("Failed to read from stdin: {}", e))?;
                Ok(content)
            } else {
                // Read from file
                fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))
            }
        }
    }
}

fn write_output(output: Option<PathBuf>, pdf_bytes: &[u8]) -> Result<(), String> {
    match output {
        None => {
            // Write to stdout
            io::stdout()
                .write_all(pdf_bytes)
                .map_err(|e| format!("Failed to write to stdout: {}", e))?;
            Ok(())
        }
        Some(path) => {
            // Write to file
            fs::write(&path, pdf_bytes)
                .map_err(|e| format!("Failed to write to file '{}': {}", path.display(), e))
        }
    }
}

fn main() {
    let args = Args::parse();

    // Read input
    let markdown = match read_input(args.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Parse page size
    let page_size = match mdpdf::config::PageSize::parse(&args.page_size) {
        Ok(size) => size,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    // Parse margin
    let margins = match mdpdf::config::Margins::parse(&args.margin) {
        Ok(margins) => margins,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Create configuration
    let config = MdpdfConfig {
        page_size: Some(page_size),
        margins: Some(margins),
        font_family: Some("Libertinus Serif".to_string()),
        font_paths: args.font_paths,
        font_size: Some(args.font_size),
        header: None,
        footer: None,
        image_handling: Some(mdpdf::config::ImageHandlingConfig::default()),
        custom_preamble: None,
        toc: args.toc,
    };

    // Convert markdown to PDF
    let (typst_code, image_files) = match markdown_to_typst(&markdown, &config) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error converting markdown to Typst: {}", e);
            process::exit(1);
        }
    };

    let pdf_bytes: Vec<u8> = match typst_to_pdf(&typst_code, &config, image_files) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error converting Typst to PDF: {}", e);
            process::exit(1);
        }
    };

    // Write output
    if let Err(e) = write_output(args.output, &pdf_bytes) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
