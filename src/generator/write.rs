//! # Atomic File Writing
//!
//! Provides atomic file writing utilities to prevent corruption during system crashes.
//! Uses the atomicwrites crate for safe temp-then-rename operations.

use super::error::GeneratorError;
use super::validate::validate_no_unsubstituted;
use atomicwrites::{AtomicFile, OverwriteBehavior};
use std::path::Path;

/// Write configuration content to a file atomically.
///
/// This function ensures that the file is written safely using the atomicwrites
/// crate, which writes to a temporary file first and then atomically renames
/// it to the target path. This prevents corruption if the system crashes
/// during the write operation.
///
/// # Arguments
///
/// * `path` - The target path where the file should be written.
/// * `content` - The content to write to the file.
///
/// # Returns
///
/// Returns `Ok(())` if the file was written successfully.
/// Returns `Err(GeneratorError::FileWrite)` if the write operation fails.
pub fn write_config_atomically(path: &Path, content: &str) -> Result<(), GeneratorError> {
    // Ensure parent directories exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| GeneratorError::FileWrite {
            path: parent.to_path_buf(),
            source: e,
        })?;
    }

    // Write atomically using atomicwrites crate
    let atomic_file = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);

    atomic_file
        .write(|f| {
            use std::io::Write;
            f.write_all(content.as_bytes())
        })
        .map_err(|e| GeneratorError::FileWrite {
            path: path.to_path_buf(),
            source: e.into(),
        })?;

    Ok(())
}

/// Write configuration content with validation before writing.
///
/// This is the primary interface for writing generated configuration files.
/// It first validates that no template placeholders remain unsubstituted,
/// then writes the content atomically.
///
/// # Arguments
///
/// * `path` - The target path where the file should be written.
/// * `content` - The content to write to the file.
/// * `template_name` - The name of the template for error messages.
///
/// # Returns
///
/// Returns `Ok(())` if validation passed and the file was written successfully.
/// Returns `Err(GeneratorError::UnsubstitutedPlaceholders)` if unsubstituted
/// placeholders are found.
/// Returns `Err(GeneratorError::FileWrite)` if the write operation fails.
pub fn write_config(path: &Path, content: &str, template_name: &str) -> Result<(), GeneratorError> {
    // Validate before writing - fail fast on unsubstituted templates
    validate_no_unsubstituted(content, template_name)?;

    // Write atomically
    write_config_atomically(path, content)?;

    Ok(())
}

/// Write multiple generated files atomically.
///
/// This helper function iterates over a slice of generated files and writes
/// each one atomically. It validates each file before writing.
///
/// # Arguments
///
/// * `files` - A slice of tuples containing (path, content, template_name).
/// * `base_path` - Optional base path to prepend to each file path.
///
/// # Returns
///
/// Returns `Ok(Vec<std::path::PathBuf>)` with the paths of all written files.
/// Returns on the first error encountered.
pub fn write_all_configs<'a>(
    files: impl IntoIterator<Item = (&'a Path, &'a str, &'a str)>,
) -> Result<Vec<std::path::PathBuf>, GeneratorError> {
    let mut written_paths = Vec::new();

    for (path, content, template_name) in files {
        write_config(path, content, template_name)?;
        written_paths.push(path.to_path_buf());
    }

    Ok(written_paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_write_config_atomically() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.nix");

        let content = r#"{
  hostname = "test-host";
}"#;

        let result = write_config_atomically(&test_file, content);
        assert!(result.is_ok());

        // Verify file was written with correct content
        let written_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(written_content, content);
    }

    #[test]
    fn test_write_config_creates_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("nested").join("directory");
        let test_file = nested_dir.join("test.nix");

        let content = r#"test = "content";"#;

        let result = write_config_atomically(&test_file, content);
        assert!(result.is_ok());
        assert!(nested_dir.exists());
    }

    #[test]
    fn test_write_config_validates_placeholders() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.nix");

        let content = r#"{
  hostname = "{{ hostname }}";
}"#;

        let result = write_config(&test_file, content, "test.nix");
        assert!(result.is_err());
        matches!(result, Err(GeneratorError::UnsubstitutedPlaceholders { .. }));
    }

    #[test]
    fn test_write_config_success() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("flake.nix");

        let content = r#"{
  description = "test - NixOS configuration";
}"#;

        let result = write_config(&test_file, content, "flake.nix");
        assert!(result.is_ok());

        let written_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(written_content, content);
    }
}
