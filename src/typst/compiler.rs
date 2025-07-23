use crate::config::MdpdfConfig;
use crate::typst::MdpdfWorld;
use anyhow::Result;
use std::collections::HashMap;

pub struct TypstCompiler;

impl TypstCompiler {
    pub fn compile_to_pdf(
        typst_code: String,
        config: MdpdfConfig,
        image_files: HashMap<String, Vec<u8>>,
    ) -> Result<Vec<u8>> {
        // Create a Typst world with the configuration, main Typst code, and image files
        let world = MdpdfWorld::new(config, typst_code, image_files);
        world.compile_to_pdf()
    }

    pub fn create_document_template(config: &MdpdfConfig) -> String {
        // Use an empty main_code and files for template generation
        let world = MdpdfWorld::new(config.clone(), String::new(), HashMap::new());
        world.create_document_template()
    }
}
