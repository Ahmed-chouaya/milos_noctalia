# Stack Research: NixOS TUI Installer

**Domain:** Terminal User Interface (TUI) installer for NixOS with flake configuration
**Researched:** January 31, 2026
**Confidence:** HIGH

## Recommended Stack

### Core TUI Framework

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **ratatui** | 0.30.0 (Dec 2025) | Terminal UI framework | De facto standard for Rust TUI; 38M+ downloads; actively maintained fork of tui-rs; immediate mode rendering; rich widget ecosystem |
| **crossterm** | 0.29.0 | Terminal backend | Recommended Ratatui backend; cross-platform (Linux/macOS/Windows); async support; no optional dependencies for core functionality |
| **tui-textarea** | 0.7.0 | Multi-line text input | Best-in-class text editing widget for Ratatui; Emacs-like shortcuts; syntax highlighting support; 63K downloads/month |
| **tui-prompts** | 0.5.0 | High-level interactive prompts | Ratatui-native prompts (text, confirm, select); inspired by JavaScript's prompts library; beautiful defaults |

### Input & Validation

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **dialoguer** | 0.12.0 | Simple CLI dialogs | 685K downloads/month; battle-tested; Confirm, Input, Select, MultiSelect, Password prompts; minimal dependencies |
| **validator** | 0.21 | Struct validation | Derive-based validation; email, length, regex, custom validators; integrates with Serde; most popular validation crate |
| **regex** | 1.11 | Pattern matching | Standard regex library; UTF-8 aware; Unicode support; used by validator crate |

### Configuration Generation

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **askama** | 0.12 | Compile-time templates | Type-safe; no runtime template parsing; Jinja2-like syntax; auto-escaping; 19.7M+ downloads |
| **serde** | 1.0 | Serialization framework | De facto standard; derives for most data types; JSON/YAML/TOML support; integrates with validator |

### Error Handling & Debugging

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **color-eyre** | 0.6 | Error reporting | Beautiful terminal error messages; captures backtraces; panic hooks; designed for CLI/TUI apps |
| **tracing** | 0.1 | Application instrumentation | Structured logging; async-aware; compatible with color-eyre; can log to file for debugging |

### Testing

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **crossterm** | 0.29.0 | Integration testing | Can mock stdin for TUI testing; enables deterministic event sequences |
| **insta** | 1.41 | Snapshot testing | Ratatui recommends for widget state testing; auto-update snapshots; diff view on failures |
| **tempfile** | 3.13 | Test fixtures | Safe temporary file creation; automatic cleanup; cross-platform |

## Installation

```nix
# flake.nix - NixOS Installer Dependencies
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay ];
      };
      rustVersion = pkgs.rust-bin.stable."1.75.0";
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustVersion
          cargo-nextest # Better test runner
        ];
        RUSTFLAGS = "-C target-cpu=native";
      };
    };
}
```

```toml
# Cargo.toml - Rust Dependencies
[package]
name = "nixos-tui-installer"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core TUI
ratatui = "0.30"
crossterm = "0.29"
tui-textarea = "0.7"
tui-prompts = "0.5"

# Simple dialogs
dialoguer = "0.12"

# Validation & Patterns
validator = { version = "0.21", features = ["derive"] }
regex = "1.11"

# Configuration Generation
askama = "0.12"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error Handling
color-eyre = "0.6"
thiserror = "2.0"

# Async (for network calls if needed)
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
tempfile = "3.13"
insta = "1.41"
crossterm = "0.29"

[profile.release]
strip = true
lto = true
codegen-units = 1
```

## Alternatives Considered

| Category | Recommended | Alternative | When to Use Alternative |
|----------|-------------|-------------|------------------------|
| TUI Framework | ratatui | tui-rs (deprecated) | Only if maintaining existing tui-rs code |
| Backend | crossterm | termion | For Wezterm-specific optimizations; termion is pure Rust but less maintained |
| Backend | crossterm | termwiz | For WezTerm users needing advanced features |
| Text Input | tui-textarea | tui-input | tui-input is simpler but less featured |
| Prompts | tui-prompts | cursive | Full widget toolkit but more opinionated; different programming model |
| Validation | validator | garde | More modern API; supports more validation types |
| Template Engine | askama | tera | Tera is runtime-evaluated; more features but slower |
| Error Handling | color-eyre | anyhow | anyhow is simpler but less powerful for CLI apps |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| **tui-rs** | Deprecated; no longer maintained; last release 2023 | ratatui (actively maintained fork with same API) |
| **cursive** | Different programming model (retained mode); harder to integrate with modern async code | ratatui (immediate mode; more flexible) |
| **ncurses-rs** | Low-level; no widget abstractions; cross-platform pain | ratatui (high-level widgets; cross-platform) |
| **termbox** | Unmaintained; limited features | crossterm + ratatui |
| **urwid** | Python library; wrong language for Nix ecosystem | ratatui (Rust native) |

## Stack Patterns by Variant

**If the installer runs in a minimal live environment:**
- Use static linking (`cargo build --release --target x86_64-unknown-linux-musl`)
- Minimize runtime dependencies
- Consider cargo-bundle for single-binary deployment

**If the installer needs advanced navigation (tabs, wizard flow):**
- Use tui-prompts for multi-step wizard pattern
- Implement state machine for step progression
- Use Block widgets for visual separation

**If the installer collects sensitive data (passwords):**
- Use dialoguer Password input with hidden characters
- Zeroize sensitive data after use
- Consider using dialoguer-ext for clipboard options

**If the installer needs to display long content (EULAs, logs):**
- Use tui-textarea with read-only mode
- Implement scrolling with scrollbar widget
- Consider pagination for terminal without mouse support

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| ratatui 0.30 | crossterm 0.27+, termion 4.0+, termwiz 0.22+ | Use crossterm for best compatibility |
| tui-textarea 0.7 | ratatui 0.26+, tui-rs 0.19+ | Check feature flags for backend support |
| tui-prompts 0.5 | ratatui 0.26+ | Uses internal state management |
| dialoguer 0.12 | console 0.16+ | Console handles terminal capabilities |
| validator 0.21 | serde 1.0+ | Validation integrates with serde derive |
| askama 0.12 | serde 1.0+ | Templates can use serde serialization |

## NixOS Integration Tools

For integration with existing NixOS tooling:

| Tool | Purpose | Integration Point |
|------|---------|-------------------|
| **disko** | Declarative disk partitioning | Generate disko config from installer; call nix-installer or nixos-anywhere |
| **nixos-anywhere** | Network-based installation | SSH into target; deploy configuration via flakes |
| **nix-installer** | Reference installer patterns | Study its architecture; not designed for TUI customization |
| **flake-file** | Flake generation from modules | Can generate flake.nix from installer choices |

## Project Structure Recommendation

```
nixos-tui-installer/
├── Cargo.toml
├── src/
│   ├── main.rs                    # Entry point, error handling setup
│   ├── app/
│   │   ├── mod.rs                 # App state and event loop
│   │   ├── state.rs               # Form data (hostname, username, etc.)
│   │   └── validation.rs          # Validation rules
│   ├── ui/
│   │   ├── mod.rs                 # UI rendering
│   │   ├── widgets/               # Custom widgets
│   │   │   ├── input.rs           # Form field widgets
│   │   │   └── navigation.rs      # Step navigation
│   │   └── screens/               # Screen definitions
│   │       ├── welcome.rs
│   │       ├── hostname.rs
│   │       ├── username.rs
│   │       ├── locale.rs
│   │       ├── paths.rs
│   │       ├── git.rs
│   │       └── review.rs
│   ├── config/
│   │   ├── mod.rs                 # Configuration generation
│   │   ├── templates/             # Askama templates
│   │   │   ├── flake.nix.j2
│   │   │   ├── configuration.nix.j2
│   │   │   └── hardware.nix.j2
│   │   └── nix_modules/           # Nix module snippets
│   └── tui.rs                     # Terminal initialization
└── tests/
    ├── integration.rs             # Full workflow tests
    └── validation.rs              # Validation unit tests
```

## Sources

- [Ratatui 0.30.0 - crates.io](https://crates.io/crates/ratatui/0.30.0) — Core TUI framework
- [Ratatui Backend Comparison](https://ratatui.rs/concepts/backends/comparison/) — Crossterm recommended for most use cases
- [tui-textarea 0.7.0 - docs.rs](https://docs.rs/tui-textarea/0.7.0) — Text input widget
- [dialoguer 0.12.0 - docs.rs](https://docs.rs/dialoguer/0.12.0) — CLI prompts
- [validator 0.21 - crates.io](https://crates.io/crates/validator/0.21) — Struct validation
- [askama 0.12 - crates.io](https://crates.io/crates/askama/0.12) — Compile-time templates
- [nix-installer - crates.io](https://crates.io/crates/nix-installer/0.23.0) — Reference installer implementation
- [disko - GitHub](https://github.com/nix-community/disko) — Declarative disk partitioning
- [flake-file - GitHub](https://github.com/vic/flake-file) — Flake generation from modules

---

*Stack research for NixOS TUI installer project*
*Researched: January 31, 2026*
