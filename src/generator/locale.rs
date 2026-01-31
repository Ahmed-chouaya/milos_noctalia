//! Locale and keyboard configuration generator.
//!
//! This module generates locale and keyboard configuration files.

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;

#[derive(Debug)]
pub struct LocaleGenerator;

impl Generator for LocaleGenerator {
    fn generate(&self, _config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        // TODO: Implement locale configuration generation
        Ok(vec![])
    }

    fn output_base_path(&self, _config: &UserConfig) -> std::path::PathBuf {
        std::path::PathBuf::from("/etc/nixos")
    }
}
