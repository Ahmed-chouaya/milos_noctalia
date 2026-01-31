//! Users and groups configuration generator.
//!
//! This module generates user and group configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;

#[derive(Template)]
#[template(path = "users.nix", escape = "none")]
struct UsersContext {
    username: String,
    full_name: String,
    git_email: String,
    git_username: String,
}

#[derive(Debug)]
pub struct UsersGenerator;

impl UsersGenerator {
    /// Get the template name for validation.
    pub fn template_name(&self) -> &'static str {
        "users.nix"
    }
}

impl Generator for UsersGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = UsersContext {
            username: config.username.clone(),
            full_name: config.full_name.clone(),
            git_email: config.git_email.clone(),
            git_username: config.git_username.clone(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("users.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("modules")
    }

    fn template_name(&self) -> &'static str {
        "users.nix"
    }
}
