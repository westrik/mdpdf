#![no_main]
use libfuzzer_sys::fuzz_target;
use mdpdf::{markdown_to_typst, typst_to_pdf, config::MdpdfConfig};

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string, handling invalid UTF-8
    if let Ok(markdown) = String::from_utf8(data.to_vec()) {
        let config = MdpdfConfig::default();
        
        // First, convert markdown to typst
        let (typst_code, image_files) = markdown_to_typst(&markdown, &config)
            .expect("markdown_to_typst should not fail");
        
        // Then, try to convert typst to PDF
        // Assert that it doesn't return an error
        let result = typst_to_pdf(&typst_code, &config, image_files);
        assert!(result.is_ok(), "typst_to_pdf returned error: {:?}", result.err());
    }
});
