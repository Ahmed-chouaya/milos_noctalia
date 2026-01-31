//! # Template Validation
//!
//! Validates generated configuration content to ensure templates were properly substituted
//! and Nix syntax is valid.

use super::error::GeneratorError;

/// Check for unsubstituted template variables in generated content.
///
/// This function scans the content for `{{` and `}}` patterns that indicate
/// template variables were not substituted during rendering.
///
/// # Arguments
///
/// * `content` - The generated configuration content to check
/// * `template_name` - Optional name of the template for error messages
///
/// # Returns
///
/// Returns `Ok(())` if no unsubstituted placeholders are found.
/// Returns `Err(GeneratorError::UnsubstitutedPlaceholders)` if placeholders remain.
///
/// # Examples
///
/// ```
/// let content = "hostname = \"my-host\";";
/// validate_no_unsubstituted(content, "flake.nix").unwrap();
/// ```
///
/// ```
/// let content = "hostname = \"{{ hostname }}\";";
/// assert!(validate_no_unsubstituted(content, "flake.nix").is_err());
/// ```
pub fn validate_no_unsubstituted(content: &str, template_name: &str) -> Result<(), GeneratorError> {
    // Find all {{ ... }} patterns
    let placeholder_pattern = regex::Regex::new(r"\{\{[^}]+\}\}")?;

    let matches: Vec<_> = placeholder_pattern.find_iter(content).collect();

    if !matches.is_empty() {
        let placeholders: Vec<_> = matches.iter().map(|m| m.as_str()).collect();
        let placeholders_str = placeholders.join(", ");

        Err(GeneratorError::UnsubstitutedPlaceholders {
            template: template_name.to_string(),
            placeholders: placeholders_str,
        })
    } else {
        Ok(())
    }
}

/// Basic Nix syntax validation for generated content.
///
/// Performs lightweight syntax checks that don't require full Nix evaluation:
/// - Balanced braces
/// - Balanced quotes
/// - No obvious syntax errors
///
/// Full Nix syntax validation should be done during `nixos-rebuild`.
///
/// # Arguments
///
/// * `content` - The generated Nix configuration content to validate
///
/// # Returns
///
/// Returns `Ok(())` if basic syntax checks pass.
/// Returns `Err(GeneratorError::NixSyntax)` if syntax errors are detected.
pub fn validate_nix_syntax(content: &str) -> Result<(), GeneratorError> {
    // Check for balanced braces
    let mut brace_depth = 0;
    let mut in_string = false;
    let mut escaped = false;

    for (line_num, line) in content.lines().enumerate() {
        for (char_num, c) in line.chars().enumerate() {
            if escaped {
                escaped = false;
                continue;
            }

            if c == '\\' && in_string {
                escaped = true;
                continue;
            }

            if c == '"' && !escaped {
                in_string = !in_string;
                continue;
            }

            // Skip brace counting inside strings
            if in_string {
                continue;
            }

            if c == '{' {
                brace_depth += 1;
            } else if c == '}' {
                if brace_depth == 0 {
                    return Err(GeneratorError::NixSyntax {
                        message: format!(
                            "unbalanced closing brace at line {}, character {}",
                            line_num + 1,
                            char_num + 1
                        ),
                    });
                }
                brace_depth -= 1;
            }
        }
    }

    if in_string {
        return Err(GeneratorError::NixSyntax {
            message: "unclosed string literal at end of file".to_string(),
        });
    }

    if brace_depth != 0 {
        return Err(GeneratorError::NixSyntax {
            message: format!("unbalanced braces: {} unmatched '{{'", brace_depth),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_no_substituted_clean() {
        let content = r#"{
  hostname = "my-host";
  username = "user";
}"#;
        assert!(validate_no_unsubstituted(content, "test.nix").is_ok());
    }

    #[test]
    fn test_validate_no_substituted_with_placeholders() {
        let content = r#"{
  hostname = "{{ hostname }}";
  username = "{{ username }}";
}"#;
        let result = validate_no_unsubstituted(content, "flake.nix");
        assert!(result.is_err());
        if let Err(GeneratorError::UnsubstitutedPlaceholders { template, placeholders }) = result {
            assert_eq!(template, "flake.nix");
            assert!(placeholders.contains("{{ hostname }}"));
            assert!(placeholders.contains("{{ username }}"));
        }
    }

    #[test]
    fn test_validate_nix_syntax_valid() {
        let content = r#"{
  hostname = "test";
  users = {
    foo = { };
    bar = { };
  };
}"#;
        assert!(validate_nix_syntax(content).is_ok());
    }

    #[test]
    fn test_validate_nix_syntax_unbalanced_braces() {
        let content = r#"{
  hostname = "test";
"#;
        let result = validate_nix_syntax(content);
        assert!(result.is_err());
        if let Err(GeneratorError::NixSyntax { message }) = result {
            assert!(message.contains("unbalanced braces"));
        }
    }

    #[test]
    fn test_validate_nix_syntax_ignores_strings() {
        // Braces inside strings should not count
        let content = r#"{
  example = "{ this is balanced }";
}"#;
        assert!(validate_nix_syntax(content).is_ok());
    }
}
