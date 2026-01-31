# Project Research Summary

**Project:** NixOS TUI Installer (milos_niri)
**Domain:** Terminal User Interface installer for NixOS with flake configuration
**Researched:** January 31, 2026
**Confidence:** MEDIUM-HIGH

## Executive Summary

This is a TUI-based installer for NixOS that generates flake-based configurations, targeting developers who want a guided installation experience without sacrificing the declarative nature of NixOS. The installer must bridge the gap between manual, complex NixOS installation and user-friendly installer experiences while respecting NixOS's core philosophy of version-controlled, reproducible configurations.

The recommended approach uses Rust with Ratatui as the TUI framework, leveraging existing ecosystem tools like Disko for declarative disk partitioning and nixos-anywhere for network-based installations. The key insight from research is that this installer serves as a configuration wizard rather than a replacement for NixOS tooling—it should generate valid flake configurations and defer actual installation operations to `nixos-install`.

The primary risks are SSH connection loss after kexec (Phase 2), device naming instability causing wrong-disk selection (Phase 3), and secrets exposure in the installer environment (Phase 4). All three have clear mitigation strategies documented in PITFALLS.md. The architecture follows a four-layer pattern (TUI → State → Generator → Executor) that separates concerns and enables testing each layer independently.

## Key Findings

### Recommended Stack

The stack is a modern Rust TUI application using Ratatui 0.30.0 as the core framework with Crossterm 0.29.0 as the backend. This combination is the de facto standard for Rust terminal applications—Ratatui has 38M+ downloads and is the actively maintained fork of tui-rs. Input handling uses tui-textarea for multi-line text editing and tui-prompts for high-level interactive prompts. For simpler dialogs like confirmations and selections, dialoguer provides battle-tested components with 685K monthly downloads.

Configuration generation relies on Askama 0.12 for compile-time template rendering, producing type-safe Nix configurations without runtime parsing overhead. Validation uses the validator crate with derive macros for struct-based validation rules. Error handling employs color-eyre for beautiful terminal error messages with backtraces. The full dependency set is documented in STACK.md with version requirements and NixOS integration patterns.

**Core technologies:**
- **ratatui 0.30** — TUI framework with immediate mode rendering and rich widget ecosystem
- **crossterm 0.29** — Cross-platform terminal backend with async support
- **askama 0.12** — Compile-time templates for generating flake.nix and configuration.nix
- **color-eyre 0.6** — Error reporting with backtraces designed for CLI/TUI apps

### Expected Features

Research reveals clear expectations based on existing installers (nixos-wizard, Aegis TUI, Lassulus nixos-installer). Users expect table stakes features around system identity (hostname, user account, password), locale configuration (language, timezone, keyboard), disk operations (selection, partitioning, bootloader, filesystem, encryption), and NixOS-specific requirements (hardware detection, configuration generation, flake support, installation execution).

This project's differentiation focus is Git integration—setting up Git credentials (user.name, user.email), SSH key configuration for GitHub/GitLab, Home Manager integration for dotfiles management, and shell environment pre-configuration. These features target developers who want their development environment ready immediately after installation.

**Must have (table stakes):**
- Hostname, user account, password configuration
- Locale, timezone, keyboard layout selection
- Disk selection with partition scheme presets (use Disko)
- Bootloader configuration (systemd-boot or GRUB)
- Hardware detection via `nixos-generate-config`
- Configuration.nix and flake.nix generation
- Network connectivity verification and progress feedback
- Error handling with recovery and summary confirmation

**Should have (differentiators):**
- Git credentials configuration in system flake
- SSH key setup for GitHub/GitLab workflows
- Home Manager module generation
- Shell environment pre-configuration (starship, zsh plugins)

**Defer (v2+):**
- ZFS support (very high complexity)
- Multi-disk RAID configurations
- Remote SSH-based installation (nixos-anywhere handles this)
- Btrfs subvolumes and snapshots
- Configuration import from existing nix configs

### Architecture Approach

The architecture follows a four-layer pattern that separates presentation from business logic. The TUI Presentation Layer handles wizard navigation, form step rendering, progress display, and summary review. The State Management Layer maintains a centralized InstallationConfig store that survives UI component lifecycle. The Configuration Generation Layer creates flake.nix, configuration.nix, and disk configurations using Askama templates. The Execution Layer performs actual installation operations with rollback capability.

This separation enables testing each layer independently and prevents common anti-patterns like storing state in UI components or generating Nix on every keystroke. The recommended project structure (tui/, state/, generator/, executor/) maps directly to these layers with clear internal boundaries.

**Major components:**
1. **Wizard Navigator** — State machine managing step progression with validation gates
2. **Installation State Store** — Centralized singleton holding all configuration data
3. **Flake Template Engine** — Generates flake.nix from collected settings via Askama
4. **Disk Config Generator** — Creates Disko-format partition configurations
5. **Action-based Executor** — Runs partition, user, bootloader, and system switch operations with rollback

### Critical Pitfalls

1. **SSH Connection Loss After Kexec** — After kexec transfers to the new kernel, SSH connections drop permanently. Implement retry logic with 30-second minimum, up to 5-minute exponential backoff. Test with simulated network interruption.

2. **Device Naming Instability** — Using volatile paths like `/dev/nvme0n1` causes wrong-disk selection. Always use `/dev/disk/by-id/*` identifiers and display model/serial numbers in disk selection UI.

3. **Secrets Exposure** — Nix store is globally readable; secrets in configuration end up world-readable. Use sops-nix/agenix, store secrets outside `/nix/store`, and implement secret redaction in logs.

4. **Network Connectivity Detection Failures** — Installer may fail to maintain connectivity during flake evaluation. Implement robust detection with manual configuration options and offline mode with pre-cached inputs.

5. **Module Evaluation Failures** — Nix module system produces cryptic errors. Validate configurations early, provide user-friendly error messages, and implement linting for known problematic patterns.

## Implications for Roadmap

Based on research, the suggested phase structure follows the architecture's layered approach, building from foundation through execution:

### Phase 1: Core Infrastructure
**Rationale:** Establishes the data model and TUI framework before any user-facing features. State store must exist before form steps can collect data.

**Delivers:** Working TUI framework with basic navigation, state management system, and project structure.

**Addresses:** Network detection pitfalls (can be tested early), progress/feedback UI patterns.

**Avoids:** Anti-pattern of storing state in UI components.

### Phase 2: Configuration System
**Rationale:** Template engine and Nix configuration generation are prerequisites for any meaningful installation. Module evaluation error handling must be built alongside generation.

**Delivers:** Flake.nix and configuration.nix generation from state data, validation with user-friendly errors.

**Uses:** askama templates, validator crate.

**Implements:** Generator layer from architecture.

**Research Flags:** Needs deeper research on flake module imports and Home Manager integration patterns.

### Phase 3: Disk Partitioning
**Rationale:** Disk operations are complex and have the highest data-loss risk. Must implement device naming stability (by-id paths) and Disko integration before any destructive operations.

**Delivers:** Disk selection UI showing model/serial, partition scheme selection, Disko config generation.

**Avoids:** Device naming instability pitfall by requiring by-id paths; wrong-disk selection by showing verification UI.

**Research Flags:** Skip research—Disko documentation is comprehensive and patterns are well-established.

### Phase 4: User & Git Configuration
**Rationale:** User creation follows disk setup logically, and Git integration is the key differentiator. Secrets handling must be correct from the start.

**Delivers:** User account creation, sudo access configuration, Git credentials setup, SSH key configuration.

**Avoids:** Secrets exposure pitfall by implementing proper secrets management from day one.

**Research Flags:** Needs research on sops-nix/agenix integration patterns.

### Phase 5: Installation Execution
**Rationale:** Execution depends on all previous phases. Must implement action-based pattern with rollback capability before running any nixos-install operations.

**Delivers:** Partition execution, user creation, bootloader installation, nixos-install orchestration, progress reporting.

**Avoids:** SSH connection loss after kexec by implementing proper retry logic; partial write failures by implementing rollback.

### Phase 6: Integration & Polish
**Rationale:** Integration testing, error recovery, and UX refinement can only happen after all components exist.

**Delivers:** Full installer workflow testing, error handling with recovery, summary view, documentation.

### Phase Ordering Rationale

- **State → TUI → Generators → Executors** follows architectural dependencies
- **Disk operations before user creation** because disk is the foundation; user creation depends on knowing where `/home` will be
- **Git integration in Phase 4** because it requires user configuration to be complete first
- **Execution last** because it depends on all generated configurations
- **Secrets handling early** (Phase 4) because retrofitting security is harder than building it in

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** Module imports and flake output types—Nix module system has complex semantics
- **Phase 4:** Secrets management with sops-nix/agenix—enterprise patterns need validation
- **Phase 5:** SSH-based installation patterns—if supporting remote deployment

Phases with standard patterns (skip research-phase):
- **Phase 1:** Ratatui and state machine patterns are well-documented
- **Phase 3:** Disko has excellent documentation and established patterns
- **Phase 6:** Integration testing follows standard Rust patterns

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Official crate documentation, established patterns, high download counts |
| Features | MEDIUM-HIGH | Community consensus from existing installers, clear table stakes |
| Architecture | MEDIUM | Patterns documented but implementation details vary by project scope |
| Pitfalls | MEDIUM-HIGH | Well-documented with real issue references, clear mitigation strategies |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- **Home Manager integration:** FEATURES.md suggests it as a differentiator but research doesn't fully specify implementation patterns. Needs validation during Phase 2 planning.

- **SSH-based installation:** If supporting remote installation, nixos-anywhere integration needs additional research beyond the current document references.

- **Multi-user vs single-user scenarios:** Research assumes single-user workstation installation. Server scenarios may need different defaults for SSH, firewall, and user configuration.

## Sources

### Primary (HIGH confidence)
- [Ratatui 0.30.0 - crates.io](https://crates.io/crates/ratatui/0.30.0) — Core TUI framework documentation
- [NixOS Manual - Installation](https://nixos.org/manual/nixos/stable/#ch-installation) — Official installation requirements
- [NixOS Manual - Configuration Options](https://nixos.org/manual/nixos/stable/options) — Option reference for configuration generation
- [nix-installer Architecture](https://docs.rs/nix-installer/latest/nix_installer/) — Action/Planner pattern reference

### Secondary (MEDIUM confidence)
- [nixos-wizard (km-clay)](https://github.com/km-clay/nixos-wizard) — Existing TUI installer patterns
- [Aegis TUI](https://github.com/Athena-OS/aegis-tui) — Alternative installer approach
- [nix-community/disko](https://github.com/nix-community/disko) — Disk configuration format and usage
- [nix-community/nixos-anywhere](https://github.com/nix-community/nixos-anywhere) — Network installation reference

### Tertiary (LOW confidence)
- [nixos-facter](https://github.com/nix-community/nixos-facter) — Hardware detection, limited documentation
- [flake-file](https://github.com/vic/flake-file) — Flake generation from modules, early alpha

### Pitfall References (specific issues)
- [nix-community/nixos-anywhere Issue #112](https://github.com/nix-community/nixos-anywhere/issues/112) — SSH loss after kexec
- [nix-community/disko Issue #551](https://github.com/nix-community/disko/issues/551) — Device naming issues
- [nix-community/disko Issue #743](https://github.com/nix-community/disko/issues/743) — Wrong boot partition

---

*Research completed: January 31, 2026*
*Ready for roadmap: yes*
