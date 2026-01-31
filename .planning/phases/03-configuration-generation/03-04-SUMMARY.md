---
phase: 03-configuration-generation
plan: "03-04"
subsystem: generator
tags: [rust, configuration, orchestration, cli]
completed: 2026-01-31
duration: "~5 minutes"
---

# Phase 3 Plan 4: Generator Orchestration Summary

**One-liner:** Generator orchestrator with `generate_all()` entry point and CLI demo mode

## Objective

Created the generator orchestrator that ties all 7 generators together and provides an entry point for the TUI. The `generate_all()` function takes UserConfig (from WizardState) and output directory, runs all 7 generators, validates each output, and writes all files atomically.

## What Was Delivered

### 1. Generator Orchestration Infrastructure

**`src/generator/mod.rs`** - Already contained:
- `all_generators()` function returning 7 boxed Generator trait objects
- `generate_all()` function that:
  - Creates output directory if needed
  - Iterates through all generators
  - Renders templates
  - Validates generated content
  - Writes files atomically to correct locations (root or modules/ subdirectory)

### 2. CLI Demo Mode

**`src/main.rs`** - Added `--generate` flag:
- Parses `--generate` flag for CLI mode
- Creates sample UserConfig with test data
- Calls `generator::generate_all(&config, &output_dir)`
- Prints generated file paths
- Default output directory: `milos-output` (can override via CLI arg)

Usage:
```bash
cargo run -- --generate              # Generate to milos-output/
cargo run -- --generate /path/to/dir # Generate to custom directory
```

### 3. Generator Trait Fixes

**`src/generator/error.rs`** - Fixed FileWrite variant:
- Changed from tuple variant to named fields for better error messages
- `GeneratorError::FileWrite { path, source }`

**`src/generator/flake.rs`, `git.rs`, `users.rs`** - Added `template_name()`:
- Implements Generator trait correctly
- Required for validation error messages

## Key Files Modified

| File | Changes |
|------|---------|
| `src/generator/mod.rs` | `all_generators()`, `generate_all()` (already present) |
| `src/generator/error.rs` | Fixed FileWrite variant to named fields |
| `src/generator/flake.rs` | Added `template_name()` method |
| `src/generator/git.rs` | Added `template_name()` method |
| `src/generator/users.rs` | Added `template_name()` method |
| `src/main.rs` | Added `--generate` flag and `run_generate_command()` |

## Verification

The implementation meets all success criteria:

- ✅ `generate_all()` function exists and orchestrates all 7 generators
- ✅ `all_generators()` returns all Generator implementations
- ✅ Validation runs before each file write (via `generator.validate()`)
- ✅ Atomic writing produces files in correct locations (root or modules/)
- ✅ CLI demo in main.rs shows generator usage via `--generate` flag

## Output Specification

After `cargo run -- --generate`, the function produces:
- `flake.nix` - Root directory
- `modules/users.nix` - Users configuration
- `modules/git.nix` - Git configuration  
- `modules/locale.nix` - Locale configuration
- `modules/noctalia.nix` - Noctalia shell configuration
- `modules/niri/config.kdl` - Niri compositor config
- `nix.conf` - Nix configuration

## Dependencies

This plan depends on:
- **03-01**: Generator module infrastructure (Generator trait, UserConfig)
- **03-02**: Askama templates for all 7 config files
- **03-03**: Validation and atomic file writing utilities

## Next Steps

Ready for **03-05**: Generator integration with TUI wizard flow. The TUI can now call `generator::generate_all()` when user completes the wizard and clicks "Generate".

## Decisions Made

1. **CLI demo approach**: Used `std::env::args()` for flag parsing instead of adding clap dependency, keeping the binary lightweight for TUI usage.

2. **Output directory structure**: Files with `output_base_path() == "."` go to root, others go to their respective subdirectories (modules/, niri/).

3. **Named fields for FileWrite**: Changed from tuple to named fields for better error messages and clearer code.
