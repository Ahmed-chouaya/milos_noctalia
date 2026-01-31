// Noctalia shell configuration generator.
//!
//! This module generates Noctalia shell configuration files.

use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

#[derive(Debug)]
pub struct NoctaliaGenerator;

impl NoctaliaGenerator {
    /// Validate generated noctalia.nix content.
    ///
    /// # Arguments
    /// * `content` - The generated content to validate.
    ///
    /// # Returns
    /// Returns `Ok(())` if no unsubstituted placeholders remain.
    pub fn validate(&self, content: &str) -> Result<(), GeneratorError> {
        validate_no_unsubstituted(content, "noctalia.nix")
    }
}

impl Generator for NoctaliaGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        // Manually render the noctalia.nix template
        let content = format!(r#"{{ inputs, config, lib, ... }}:
{{
  # Noctalia shell configuration

  # Wallpaper directory
  home.file.".config/noctalia/wallpapers".source = "{}";

  # Noctalia settings
  programs.noctalia = {{
    enable = true;
    wallpaperDir = "{}";
  }};
}}"#, config.wallpaper_dir, config.wallpaper_dir);

        // Validate no unsubstituted placeholders remain
        self.validate(&content)?;

        Ok(vec![GeneratedFile {
            path: PathBuf::from("noctalia.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("modules")
    }

    fn template_name(&self) -> &'static str {
        "noctalia.nix"
    }
}
