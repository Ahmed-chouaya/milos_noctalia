//! Noctalia shell configuration generator.
//!
//! This module generates Noctalia shell configuration files.

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;

#[derive(Debug)]
pub struct NoctaliaGenerator;

impl Generator for NoctaliaGenerator {
    fn generate(&self, _config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        // TODO: Implement Noctalia configuration generation
        Ok(vec![])
    }

    fn output_base_path(&self, _config: &UserConfig) -> std::path::PathBuf {
        std::path::PathBuf::from("/etc/nixos")
    }
}
