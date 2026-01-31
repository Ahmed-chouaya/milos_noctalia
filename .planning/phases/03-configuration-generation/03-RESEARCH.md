# Phase 3: Configuration Generation - Research

**Researched:** 2026-01-31
**Domain:** Template substitution engine for NixOS flake and module files using Askama
**Confidence:** HIGH

## Summary

Phase 3 implements a template substitution engine using Askama 0.15.x (compile-time validated Jinja-like templates) to generate NixOS configuration files from user-collected data. The research confirms Askama as the right choice: it provides compile-time template validation, type-safe context structs, and clean error handling that maps well to the generator pattern.

The key insight is that Askama performs template syntax validation at compile time through its derive macro, catching missing variables, type mismatches, and template syntax errors before runtime. This eliminates an entire class of runtime errors. For file generation, the atomicwrites crate provides safe file writing that prevents partial file corruption on system crashes.

**Primary recommendation:** Use Askama 0.15.x with context structs matching each configuration file, validate all templates at instantiation time, and write files atomically using the atomicwrites crate with temp-then-rename pattern.

## Standard Stack

The established libraries and versions for configuration generation:

### Core Dependencies
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| askama | 0.15.1 | Compile-time template engine | Type-safe, Jinja-like syntax, excellent error messages |
| atomicwrites | 0.4.4 | Atomic file writing | Cross-platform, prevents corruption on crash |
| thiserror | 1.0 | Error type derivation | Clean error enum definitions with From impls |
| anyhow | 1.0 | Context-rich error handling | ? operator propagation with source context |

### Feature Flags
```toml
[dependencies]
askama = "0.15"
atomicwrites = "0.4"
thiserror = "1.0"
anyhow = "1.0"
```

### Cargo Configuration
No special features required for basic template rendering. Askama's default configuration handles escaping based on file extension.

**Installation:**
```bash
cargo add askama@=0.15 atomicwrites@=0.4 thiserror anyhow
```

## Architecture Patterns

### Recommended Project Structure

```
milos-generator/
├── src/
│   ├── main.rs              # Entry point, orchestrates generation
│   ├── lib.rs               # Library interface
│   ├── error.rs             # Error types and handling
│   ├── context.rs           # User data structures (from Phase 2)
│   ├── templates/
│   │   ├── flake.nix        # Flake template
│   │   ├── users.nix        # User configuration
│   │   ├── git.nix          # Git credentials
│   │   ├── locale.nix       # Timezone/keyboard
│   │   ├── noctalia.nix     # Wallpaper/avatar
│   │   ├── niri/
│   │   │   └── config.kdl   # Niri screenshot path
│   │   └── nix.conf         # Trusted users
│   └── generator/
│       ├── mod.rs           # Generator trait/interface
│       ├── flake.rs         # Flake generator
│       ├── users.rs         # Users generator
│       ├── git.rs           # Git generator
│       ├── locale.rs        # Locale generator
│       ├── noctalia.rs      # Noctalia generator
│       ├── niri.rs          # Niri generator
│       └── nixconf.rs       # Nix.conf generator
```

### Pattern 1: Askama Template Context Struct

Each configuration file has a corresponding context struct that derives Template:

```rust
// Source: https://docs.rs/askama/latest/askama/derive.Template.html

use askama::Template;

#[derive(Template)]
#[template(path = "flake.nix")]
struct FlakeContext {
    hostname: String,
    username: String,
    nixpkgs_ref: String,
}

fn generate_flake(config: &UserConfig) -> anyhow::Result<String> {
    let context = FlakeContext {
        hostname: config.hostname.clone(),
        username: config.username.clone(),
        nixpkgs_ref: "nixos-unstable".to_string(),
    };
    Ok(context.render()?)
}
```

**When to use:** Every generated file should have a dedicated context struct. This ensures compile-time validation of template variables.

### Pattern 2: Generator Trait for Uniform Processing

```rust
// Generator trait defining the interface for all configuration generators
trait Generator {
    /// Template file name (relative to templates directory)
    fn template_name(&self) -> &'static str;
    
    /// Output file path (relative to output directory)
    fn output_path(&self, context: &UserConfig) -> PathBuf;
    
    /// Generate the configuration file
    fn generate(&self, context: &UserConfig) -> anyhow::Result<GeneratedFile>;
    
    /// Validate the generated content before writing
    fn validate(&self, content: &str) -> Result<(), ValidationError>;
}

struct FlakeGenerator;

impl Generator for FlakeGenerator {
    fn template_name(&self) -> &'static str {
        "flake.nix"
    }
    
    fn output_path(&self, context: &UserConfig) -> PathBuf {
        PathBuf::from("flake.nix")
    }
    
    fn generate(&self, context: &UserConfig) -> anyhow::Result<GeneratedFile> {
        let template = FlakeContext {
            hostname: context.hostname.clone(),
            username: context.username.clone(),
            nixpkgs_ref: "nixos-unstable".to_string(),
        };
        let content = template.render()?;
        Ok(GeneratedFile {
            path: self.output_path(context),
            content,
        })
    }
    
    fn validate(&self, content: &str) -> Result<(), ValidationError> {
        // Validate Nix syntax
        if content.contains("{{") || content.contains("}}") {
            return Err(ValidationError::TemplateVariablesUnsubstituted);
        }
        // Additional validation as needed
        Ok(())
    }
}
```

**When to use:** When you have multiple configuration files with similar generation patterns. Provides uniform error handling and validation.

### Pattern 3: Atomic File Writing

```rust
// Source: https://docs.rs/atomicwrites/latest/atomicwrites/struct.AtomicFile.html

use atomicwrites::{AtomicFile, OverwriteBehavior};

fn write_config_atomically(path: &Path, content: &str) -> anyhow::Result<()> {
    let af = AtomicFile::new(
        path,
        OverwriteBehavior::AllowOverwrite,
    );
    
    af.write(|f| {
        use std::io::Write;
        f.write_all(content.as_bytes())
    })?;
    
    Ok(())
}

// Usage for directory creation + atomic write
fn generate_config_file(
    generator: &dyn Generator,
    context: &UserConfig,
    output_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let generated = generator.generate(context)?;
    
    // Create parent directories
    let full_path = output_dir.join(&generated.path);
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Validate before writing
    generator.validate(&generated.content)?;
    
    // Write atomically
    write_config_atomically(&full_path, &generated.content)?;
    
    Ok(full_path)
}
```

**When to use:** Every file write operation, especially for system configuration files. Prevents corruption if the system crashes during write.

### Pattern 4: Askama Error Handling

```rust
// Source: https://docs.rs/askama/latest/askama/enum.Error.html

use askama::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("template rendering failed: {source}")]
    TemplateError {
        #[from]
        source: Error,
    },
    
    #[error("required value missing: {field}")]
    ValueMissing {
        field: String,
    },
    
    #[error("value type mismatch: {field}")]
    ValueType {
        field: String,
        expected: &'static str,
        actual: &'static str,
    },
    
    #[error("validation failed: {message}")]
    Validation {
        message: String,
    },
    
    #[error("file write failed: {path}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl From<Error> for GeneratorError {
    fn from(e: Error) -> Self {
        match e {
            Error::Fmt => Self::TemplateError {
                source: e,
            },
            Error::ValueMissing => Self::ValueMissing {
                field: "unknown".to_string(),
            },
            Error::ValueType => Self::ValueType {
                field: "unknown".to_string(),
                expected: "unknown",
                actual: "unknown",
            },
            Error::Custom(_) => Self::TemplateError {
                source: e,
            },
            Error::Json(_) => Self::TemplateError {
                source: e,
            },
        }
    }
}
```

**When to use:** When you need to map Askama's errors to your application's error domain. Provides user-friendly messages.

### Pattern 5: Template Validation Before Writing

```rust
// Validate all templates at instantiation time to catch errors early
fn validate_all_templates(context: &UserConfig) -> Result<(), GeneratorError> {
    // Try rendering each template with the context
    let templates: Vec<(&str, Box<dyn Fn() -> Result<String, Error>>)> = vec![
        ("flake.nix", Box::new(|| {
            FlakeContext {
                hostname: context.hostname.clone(),
                username: context.username.clone(),
                nixpkgs_ref: "nixos-unstable".to_string(),
            }.render()
        })),
        // ... other templates
    ];
    
    for (name, render_fn) in templates {
        match render_fn() {
            Ok(content) => {
                // Check for unsubstituted placeholders
                if content.contains("{{") {
                    return Err(GeneratorError::Validation {
                        message: format!("{} has unsubstituted template variables", name),
                    });
                }
            }
            Err(e) => {
                return Err(GeneratorError::TemplateError { source: e });
            }
        }
    }
    
    Ok(())
}
```

**When to use:** Before writing any files, validate all templates can render successfully. Provides early error detection.

### Pattern 6: Nix Module Structure

```nix
# flake.nix template pattern
{
  description = "{{ hostname }} - NixOS configuration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, home-manager, ... }:
    let
      system = "x86_64-linux";
      username = "{{ username }}";
    in {
      nixosConfigurations = {
        {{ hostname }} = nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            ./modules/nixos
            ./modules/users.nix
            ./modules/git.nix
            ./modules/locale.nix
            ./modules/noctalia.nix
          ];
        };
      };
    };
}
```

**Source:** [NixOS Flakes Wiki](https://wiki.nixos.org/wiki/Flakes)

**When to use:** The flake.nix template follows this standard structure with nixosConfigurations, inputs, and outputs.

### Anti-Patterns to Avoid

- **Don't use runtime string substitution libraries (like handlebars runtime):** Askama's compile-time validation catches errors at build time
- **Don't write files without atomic operations:** Power loss during write corrupts configuration
- **Don't ignore Askama::Error variants:** All variants are non-exhaustive; handle them explicitly
- **Don't skip template validation:** Missing variables fail at render time, not compile time
- **Don't use unwrap() on render results:** Template.render() returns Result; propagate errors properly

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Template engine | Custom regex-based substitution | Askama | Compile-time validation, type safety, excellent error messages |
| File writing | fs::write() with manual error handling | atomicwrites | Atomic rename prevents corruption, cross-platform |
| Error propagation | Manual match on error types | thiserror + anyhow | Idiomatic Rust, source chain preservation |
| Template syntax validation | Manual placeholder checking | Askama derive macro | Catches errors at compile time |
| Conditional templates | String concatenation | Askama if/else/for blocks | Type-checked, no runtime syntax errors |
| Escaping special characters | Manual escaping | Askama built-in escapers | Prevents injection, context-aware |

**Key insight:** Template engines seem simple (just string replacement), but edge cases (nested conditionals, loops, filters, escaping) make building one correctly a significant undertaking. Askama handles all of this with compile-time guarantees.

## Common Pitfalls

### Pitfall 1: Template Variables Not Matching Struct Fields

**What goes wrong:** Template renders with empty strings or Askama returns ValueMissing error.

**Why it happens:** Askama requires template variables to exactly match struct field names (case-sensitive). Typos or renames break templates silently at compile time for inline templates, or at render time for file templates.

**How to avoid:** Use struct field names that match template variables exactly. Consider deriving Debug on context structs and printing them during development.

```rust
// Template: {{ hostname }} requires struct field `hostname` (not `host` or `hostName`)
#[derive(Template)]
#[template(path = "flake.nix")]
struct FlakeContext {
    hostname: String,  // Must match {{ hostname }} in template
    username: String,
}
```

**Warning signs:** Askama Error::ValueMissing at runtime, empty strings in generated files.

### Pitfall 2: Non-Exhaustive Askama Error Enum

**What goes wrong:** New Askama versions add error variants, causing compile errors or silent ignores.

**Why it happens:** Askama's Error enum is marked #[non_exhaustive], meaning pattern matches must include `_ =>` wildcard.

**How to avoid:** Always handle the wildcard case in error conversions:

```rust
impl From<Error> for GeneratorError {
    fn from(e: Error) -> Self {
        match e {
            Error::Fmt => Self::TemplateError { source: e },
            Error::ValueMissing => Self::ValueMissing { field: "unknown".to_string() },
            Error::ValueType => Self::ValueType { field: "unknown".to_string(), expected: "unknown", actual: "unknown" },
            Error::Custom(_) => Self::TemplateError { source: e },
            Error::Json(_) => Self::TemplateError { source: e },
            // Future variants fall here
            _ => Self::TemplateError { source: e },
        }
    }
}
```

**Warning signs:** Compiler warnings about non-exhaustive enum match.

### Pitfall 3: Not Validating Templates Before Writing

**What goes wrong:** Configuration files with unsubstituted {{ placeholders }} get written to disk.

**Why it happens:** Askama renders templates correctly if context structs are populated, but typos in template syntax can leave variables unrendered.

**How to avoid:** Validate generated content before writing:

```rust
fn validate_generated(content: &str) -> Result<(), GeneratorError> {
    // Check for template syntax remnants
    if content.contains("{{") || content.contains("}}") {
        return Err(GeneratorError::Validation {
            message: "Template variables not substituted".to_string(),
        });
    }
    
    // Check for common syntax errors in Nix
    if content.contains("{{ hostname }}") || content.contains("{{ username }}") {
        return Err(GeneratorError::Validation {
            message: "Missing context for template variables".to_string(),
        });
    }
    
    Ok(())
}
```

**Warning signs:** Generated files contain literal {{ variable }} strings.

### Pitfall 4: Atomic File Write to Wrong Directory

**What goes wrong:** Atomic rename fails because temp file and target are on different filesystems.

**Why it happens:** atomicwrites creates temp file in the same directory as target. If target path doesn't exist or spans filesystems, rename fails.

**How to avoid:** Ensure parent directories exist before atomic write:

```rust
fn write_config(path: &Path, content: &str) -> anyhow::Result<()> {
    // Create parent directories FIRST
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Now atomic write will work
    let af = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
    af.write(|f| f.write_all(content.as_bytes()))?;
    
    Ok(())
}
```

**Warning signs:** atomicwrites error about cross-device link, ENOTDIR.

### Pitfall 5: Missing Newline at End of File

**What goes wrong:** Generated files missing final newline, causing issues with some tools.

**Why it happens:** Templates don't automatically add trailing newlines.

**How to avoid:** Ensure templates end with newline or add it programmatically:

```rust
let content = template.render()?;
let content = if !content.ends_with('\n') {
    content + "\n"
} else {
    content
};
```

**Warning signs:** Nix flake lint warnings about missing final newline.

### Pitfall 6: Escaping Issues with Special Characters

**What goes wrong:** Special characters in user input (like $ in paths) cause Nix evaluation errors.

**Why it happens:** Nix uses $ for string interpolation. User paths containing $ need proper escaping.

**How to avoid:** Use Askama's escape filters or escape user input before rendering:

```rust
// In template, escape user-provided paths
{{ wallpaper_path | escape_nix }}

// Or create a wrapper struct that escapes on render
struct NixEscapedString<'a>(&'a str);

impl<'a> Display for NixEscapedString<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let escaped = self.0.replace('$', "\\$");
        write!(f, "{}", escaped)
    }
}
```

**Warning signs:** Nix evaluation errors mentioning "undefined variable" or syntax errors in generated paths.

## Code Examples

### Complete Template Context Struct

```rust
// Source: Adapted from Askama documentation patterns

use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "flake.nix")]
struct FlakeContext {
    hostname: String,
    username: String,
    nixpkgs_ref: String,
}

impl FlakeContext {
    fn new(hostname: &str, username: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            username: username.to_string(),
            nixpkgs_ref: "nixos-unstable".to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "users.nix")]
struct UsersContext {
    username: String,
    full_name: String,
    email: String,
    groups: Vec<&'static str>,
}

impl UsersContext {
    fn new(username: &str, full_name: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            full_name: full_name.to_string(),
            email: email.to_string(),
            groups: vec!["wheel", "sudo", "audio", "video", "users"],
        }
    }
}

#[derive(Template)]
#[template(path = "niri/config.kdl")]
struct NiriContext {
    screenshot_path: String,
}

impl NiriContext {
    fn new(screenshot_path: &str) -> Self {
        Self {
            screenshot_path: screenshot_path.to_string(),
        }
    }
}
```

### Generator Module Pattern

```rust
// src/generator/mod.rs

use anyhow::{Context, Result};
use askama::Template;
use std::path::PathBuf;

pub trait Generator {
    fn generate(&self, config: &UserConfig) -> Result<GeneratedFile>;
    fn output_path(&self, config: &UserConfig) -> PathBuf;
}

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

pub struct UserConfig {
    pub hostname: String,
    pub username: String,
    pub full_name: String,
    pub git_username: String,
    pub git_email: String,
    pub timezone: String,
    pub keyboard_layout: String,
    pub wallpaper_path: String,
    pub avatar_path: Option<String>,
    pub screenshot_path: String,
}

pub mod flake;
pub mod users;
pub mod git;
pub mod locale;
pub mod noctalia;
pub mod niri;
pub mod nixconf;
```

### Flake Generator Implementation

```rust
// src/generator/flake.rs

use super::{Generator, GeneratedFile, UserConfig};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "flake.nix")]
struct FlakeContext {
    hostname: String,
    username: String,
    nixpkgs_ref: String,
}

pub struct FlakeGenerator;

impl Generator for FlakeGenerator {
    fn generate(&self, config: &UserConfig) -> Result<GeneratedFile> {
        let context = FlakeContext {
            hostname: config.hostname.clone(),
            username: config.username.clone(),
            nixpkgs_ref: "nixos-unstable".to_string(),
        };
        
        let content = context.render()
            .context("Failed to render flake.nix template")?;
        
        Ok(GeneratedFile {
            path: self.output_path(config),
            content,
        })
    }
    
    fn output_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("flake.nix")
    }
}
```

### Validation and Atomic Write

```rust
// src/generator.rs

use anyhow::{Context, Result};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use std::path::Path;

pub fn write_config_atomically(path: &Path, content: &str) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }
    
    let af = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
    af.write(|f| {
        use std::io::Write;
        f.write_all(content.as_bytes())
    }).with_context(|| format!("Failed to write config to: {:?}", path))?;
    
    Ok(())
}

pub fn validate_template_substitution(content: &str) -> Result<()> {
    // Check for unsubstituted template variables
    let placeholder_pattern = regex::Regex::new(r"\{\{[^}]+\}\}")?;
    
    if placeholder_pattern.is_match(content) {
        anyhow::bail!(
            "Generated file contains unsubstituted template variables"
        );
    }
    
    Ok(())
}
```

### Main Orchestration

```rust
// src/main.rs

mod context;
mod generator;

use crate::generator::{Generator, UserConfig, flake::FlakeGenerator};
use anyhow::{Context, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    // Load user configuration from Phase 2
    let config = load_user_config()?;
    
    // Create output directory
    let output_dir = PathBuf::from("milos-output");
    std::fs::create_dir_all(&output_dir)?;
    
    // Initialize generators
    let generators: Vec<Box<dyn Generator>> = vec![
        Box::new(FlakeGenerator),
        Box::new(generator::users::UsersGenerator),
        Box::new(generator::git::GitGenerator),
        Box::new(generator::locale::LocaleGenerator),
        Box::new(generator::noctalia::NoctaliaGenerator),
        Box::new(generator::niri::NiriGenerator),
        Box::new(generator::nixconf::NixConfGenerator),
    ];
    
    // Generate all configurations
    for generator in generators {
        let generated = generator.generate(&config)?;
        
        // Validate before writing
        generator::validate_template_substitution(&generated.content)?;
        
        // Write atomically
        let output_path = output_dir.join(&generated.path);
        generator::write_config_atomically(&output_path, &generated.content)?;
        
        println!("Generated: {:?}", generated.path);
    }
    
    println!("Configuration generation complete!");
    Ok(())
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Runtime string replacement | Askama compile-time templates | 0.12 → 0.15 | Type safety, early error detection |
| Manual error handling | thiserror + anyhow | 2023+ | Idiomatic Rust, better error context |
| Regular file writes | atomicwrites crate | 2020+ | Prevents corruption on crash |
| Inline templates | Separate template files | Best practice | Better separation, syntax highlighting |
| Minimal validation | Pre-write validation | 2024+ | Fail fast, user-friendly errors |

**Deprecated/outdated:**
- `handlebars` runtime template engine (Askama is compile-time validated)
- `failure` crate (replaced by thiserror + anyhow)
- String concatenation for templates (Askama blocks are type-safe)
- Manual newline handling (templates should include final newline)

## Open Questions

1. **Nix escaping requirements**
   - What we know: Nix uses $ for interpolation, paths with $ need escaping
   - What's unclear: Complete list of characters that need escaping in Nix strings
   - Recommendation: Test generated configs with nix-instantiate or nix eval to validate syntax

2. **Template directory structure for cross-compilation**
   - What we know: Templates can be in a templates/ directory
   - What's unclear: Best practice for platform-specific templates (if needed)
   - Recommendation: Keep all templates in one directory, use conditional blocks for differences

3. **Validation depth for generated Nix**
   - What we know: nix-instantiate can check syntax
   - What's unclear: Whether to run full Nix evaluation during generation
   - Recommendation: Basic syntax validation only; full evaluation during nixos-rebuild

## Sources

### Primary (HIGH confidence)
- [Askama 0.15.1 Documentation](https://docs.rs/askama/0.15.1/askama/) - Official crate docs, comprehensive API reference
- [Askama Template Syntax](https://askama.rs/en/stable/template_syntax.html) - Official syntax guide
- [Askama Error Enum](https://docs.rs/askama/latest/askama/enum.Error.html) - Error variants and handling
- [AtomicWrites Crate](https://docs.rs/atomicwrites/latest/atomicwrites/) - Official atomic file writing
- [NixOS Flakes Wiki](https://wiki.nixos.org/wiki/Flakes) - Official flake structure documentation

### Secondary (MEDIUM confidence)
- [NixOS Modular Configuration Guide](https://nixos-and-flakes.thiscute.world/nixos-with-flakes/modularize-the-configuration) - Community patterns for module organization
- [flake-parts Pattern](https://discourse.nixos.org/t/pattern-every-file-is-a-flake-parts-module/61271) - Community module pattern
- [thiserror Documentation](https://docs.rs/thiserror/latest/thiserror/) - Error type derivation patterns

### Tertiary (LOW confidence)
- [Atomic File Writing Patterns](https://users.rust-lang.org/t/how-to-write-replace-files-atomically/42821) - Community discussion of atomic write approaches
- [Nix Configuration Templates](https://github.com/thiloho/nixos-flake-config-template) - Example template structures

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All libraries are well-documented, stable, and actively maintained
- Architecture: HIGH - Askama patterns are well-established, atomic writes are standard practice
- Template patterns: HIGH - Askama documentation is comprehensive
- Error handling: HIGH - thiserror/anyhow patterns are idiomatic Rust
- Nix integration: MEDIUM - Documentation exists but Nix escaping needs validation

**Research date:** 2026-01-31
**Valid until:** 2026-07-31 (6 months - Rust crates are stable but may have point releases)
