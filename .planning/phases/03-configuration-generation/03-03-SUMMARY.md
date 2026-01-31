---
phase: 03-configuration-generation
plan: "03-03"
subsystem: generator
tags: [validation, file-writing, templates, atomic-writes]
tech-stack:
  added: [regex, tempfile]
  patterns: [atomic-file-writes, template-validation]
key-files:
  created:
    - src/generator/validate.rs: Template substitution and Nix syntax validation
    - src/generator/write.rs: Atomic file writing utilities
  modified:
    - src/generator/mod.rs: Added validate and write module exports
    - src/generator/flake.rs: Added validate() method
    - src/generator/users.rs: Added validate() method
    - src/generator/git.rs: Added validate() method
    - src/generator/locale.rs: Added validate() method
    - src/generator/noctalia.rs: Added validate() method
    - src/generator/niri.rs: Added validate() method
    - src/generator/nixconf.rs: Added validate() method
    - Cargo.toml: Added regex and tempfile dependencies
---

# Phase 3 Plan 03-03: Validation and Atomic File Writing

## Objective

Implemented validation and atomic file writing utilities for generated configurations. Validates that all template variables were substituted (no `{{ placeholder }}` syntax remaining) and writes files atomically to prevent corruption.

## Summary

Successfully implemented two critical safety mechanisms for configuration generation:

1. **Template Validation** - Ensures no unsubstituted placeholders remain in generated files before writing
2. **Atomic File Writing** - Uses temp-then-rename pattern to prevent corruption on system crashes

## Tasks Completed

| Task | Name | Status |
|------|------|--------|
| 1 | Template substitution validation | ✓ Complete |
| 2 | Atomic file writing | ✓ Complete |
| 3 | Add validation to all generators | ✓ Complete |

### Task 1: Template Substitution Validation

Created `src/generator/validate.rs` with two validation functions:

- `validate_no_unsubstituted(content, template_name)` - Scans for `{{...}}` patterns indicating unsubstituted variables
- `validate_nix_syntax(content)` - Performs lightweight Nix syntax checks (balanced braces, quotes)

Both functions return `Result<(), GeneratorError>` for consistent error handling.

### Task 2: Atomic File Writing

Created `src/generator/write.rs` with atomic writing utilities:

- `write_config_atomically(path, content)` - Uses `atomicwrites` crate for safe temp-then-rename
- `write_config(path, content, template_name)` - Validates before writing, fails fast on errors
- `write_all_configs(files)` - Helper for batch writing with validation

Parent directories are created automatically before writing.

### Task 3: Generator Validation Integration

Added `validate()` method to all 7 generators:

| Generator | Template | validate() Added |
|-----------|----------|------------------|
| FlakeGenerator | flake.nix | ✓ |
| UsersGenerator | users.nix | ✓ |
| GitGenerator | git.nix | ✓ |
| LocaleGenerator | locale.nix | ✓ |
| NoctaliaGenerator | noctalia.nix | ✓ |
| NiriGenerator | niri/config.kdl | ✓ |
| NixConfGenerator | nix.conf | ✓ |

Each `validate()` method calls `validate_no_unsubstituted()` with the appropriate template name.

## Dependencies Added

- `regex = "1"` - For placeholder pattern matching in validation
- `tempfile = "4"` - Dev dependency for atomic write tests

## Decisions Made

### Validation Strategy

**Decision:** Implement lightweight validation at generation time, defer full Nix evaluation to `nixos-rebuild`

**Rationale:**
- Askama already validates template syntax at compile time
- Full Nix evaluation requires nixpkgs and is slow
- Runtime errors will be caught during the actual system configuration
- Validation here catches missing variables early with helpful error messages

### Atomic Write Approach

**Decision:** Use `atomicwrites` crate with `OverwriteBehavior::AllowOverwrite`

**Rationale:**
- Cross-platform atomic rename semantics
- Prevents partial file corruption on crash/power loss
- Simple API that handles temp file creation and cleanup
- Standard practice for configuration file writing

## Verification

- ✓ `validate_no_unsubstituted()` detects `{{ placeholder }}` patterns
- ✓ `validate_nix_syntax()` identifies unbalanced braces and unclosed strings
- ✓ `write_config_atomically()` creates parent directories before writing
- ✓ `write_config()` fails fast when templates have unsubstituted variables
- ✓ All 7 generators have `validate()` method integrated

## Metrics

- **Duration:** Plan executed in single autonomous wave
- **Files Created:** 2 (validate.rs, write.rs)
- **Files Modified:** 8 (7 generators + mod.rs + Cargo.toml)
- **Lines Added:** ~105 lines of Rust code + tests

## Next Steps

The validation and atomic writing utilities are now ready for integration with the orchestration module (03-04) and main generation pipeline.
