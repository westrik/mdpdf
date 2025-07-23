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
}

fn parse_custom_page_size(value: &str) -> Result<mdpdf::config::PageSize, String> {
    // Parse custom page sizes like "8.5inx11in" or "210mmx297mm"
    let parts: Vec<&str> = value.split('x').collect();
    if parts.len() != 2 {
        return Err(
            "Custom page size must be in format 'WIDTHxHEIGHT' (e.g., '8.5inx11in')".to_string(),
        );
    }

    let width = parse_dimension(parts[0])?;
    let height = parse_dimension(parts[1])?;

    Ok(mdpdf::config::PageSize::Custom { width, height })
}

fn parse_dimension(s: &str) -> Result<f64, String> {
    let s = s.trim();

    if s.ends_with("in") {
        let value = s[..s.len() - 2]
            .parse::<f64>()
            .map_err(|_| format!("Invalid dimension: {}", s))?;
        Ok(value)
    } else if s.ends_with("mm") {
        let value = s[..s.len() - 2]
            .parse::<f64>()
            .map_err(|_| format!("Invalid dimension: {}", s))?;
        // Convert mm to inches (1 inch = 25.4 mm)
        Ok(value / 25.4)
    } else if s.ends_with("cm") {
        let value = s[..s.len() - 2]
            .parse::<f64>()
            .map_err(|_| format!("Invalid dimension: {}", s))?;
        // Convert cm to inches (1 inch = 2.54 cm)
        Ok(value / 2.54)
    } else {
        // Assume inches if no unit specified
        s.parse::<f64>()
            .map_err(|_| format!("Invalid dimension: {}", s))
    }
}

fn parse_margin(s: &str) -> Result<f64, String> {
    parse_dimension(s)
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
    let page_size = match args.page_size.to_lowercase().as_str() {
        "letter" => mdpdf::config::PageSize::Letter,
        "legal" => mdpdf::config::PageSize::Legal,
        "a4" => mdpdf::config::PageSize::A4,
        custom if custom.contains('x') => {
            // Parse custom page size
            match parse_custom_page_size(custom) {
                Ok(size) => size,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!(
                "Error: Invalid page size '{}'. Use 'letter', 'legal', 'a4', or custom format like '8.5inx11in'",
                args.page_size
            );
            process::exit(1);
        }
    };

    // Parse margin
    let margin_size = match parse_margin(&args.margin) {
        Ok(size) => size,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Create configuration
    let config = MdpdfConfig {
        page_size: Some(page_size),
        margins: Some(mdpdf::config::Margins {
            top: margin_size,
            bottom: margin_size,
            left: margin_size,
            right: margin_size,
        }),
        font_family: Some("Libertinus Serif".to_string()),
        font_size: Some(args.font_size),
        header: None,
        footer: None,
        image_handling: Some(mdpdf::config::ImageHandlingConfig::default()),
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
