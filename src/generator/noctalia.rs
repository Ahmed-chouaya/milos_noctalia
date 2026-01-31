//! Noctalia shell configuration generator.
//!
//! This module generates Noctalia shell configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;

#[derive(Template)]
#[template(path = "noctalia.nix")]
struct NoctaliaContext {
    wallpaper_dir: String,
    avatar_path: Option<String>,
}

#[derive(Debug)]
pub struct NoctaliaGenerator;

impl Generator for NoctaliaGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = NoctaliaContext {
            wallpaper_dir: config.wallpaper_dir.clone(),
            avatar_path: config.avatar_path.clone(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("noctalia.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("modules")
    }
}
