# Requirements: milos_niri - NixOS TUI Installer

**Defined:** 2026-01-31
**Core Value:** Users can reproduce this exact desktop environment (Niri compositor + Noctalia shell + dev tools) on any NixOS machine in under 10 minutes through an interactive guided installer.

## v1 Requirements

### TUI Framework

- [ ] **TUI-01**: Initialize Ratatui project structure with Cargo
- [ ] **TUI-02**: Set up Crossterm event loop for keyboard input
- [ ] **TUI-03**: Implement centralized state management store
- [ ] **TUI-04**: Create wizard flow with steps and navigation
- [ ] **TUI-05**: Add color-eyre error handling with beautiful backtraces
- [ ] **TUI-06**: Display MILOS pixel art logo on installer startup screen
- [ ] **TUI-07**: Pixel art logo with color scheme matching Noctalia theme
- [ ] **TUI-08**: Logo animation effect (optional polish)

### Input Collection

- [ ] **INP-01**: Hostname input with validation (alphanumeric, no spaces)
- [ ] **INP-02**: Username input with validation (lowercase, alphanumeric)
- [ ] **INP-03**: Full name input (free text)
- [ ] **INP-04**: Git username input (for commit author)
- [ ] **INP-05**: Git email input (for commit author)
- [ ] **INP-06**: Timezone selection (region/city list)
- [ ] **INP-07**: Keyboard layout selection (us, fr, etc.)
- [ ] **INP-08**: Wallpaper directory path (default: ~/Pictures/Wallpapers)
- [ ] **INP-09**: Avatar image path (optional, default: none)
- [ ] **INP-10**: Screenshot path (default: ~/Pictures/Screenshots)

### Configuration Generation

- [ ] **CFG-01**: Substitute hostname in flake.nix and modules
- [ ] **CFG-02**: Substitute username in users.nix, groups, home paths
- [ ] **CFG-03**: Substitute git credentials in git.nix
- [ ] **CFG-04**: Substitute locale/timezone in locale.nix
- [ ] **CFG-05**: Substitute keyboard layout in locale.nix
- [ ] **CFG-06**: Substitute wallpaper/avatar paths in noctalia.nix
- [ ] **CFG-07**: Substitute screenshot path in niri config.kdl
- [ ] **CFG-08**: Update trusted-users in nix settings
- [ ] **CFG-09**: Validate all substitutions before writing

### Execution

- [ ] **EXEC-01**: Run `git commit` of generated configs
- [ ] **EXEC-02**: Run `nixos-rebuild switch --flake`
- [ ] **EXEC-03**: Display progress and logs in TUI
- [ ] **EXEC-04**: Handle rebuild errors with user-friendly messages
- [ ] **EXEC-05**: Offer rollback option on failure

## v2 Requirements

### Advanced Features

- **ADV-01**: Configuration export/import (backup and restore)
- **ADV-02**: SSH-based remote installation (nixos-anywhere integration)
- **ADV-03**: Disk partitioning with Disko UI
- **ADV-04**: Multi-user support
- **ADV-05**: ZFS/btrfs filesystem options

### UX Enhancements

- **UX-01**: Theme selection (light/dark/custom)
- **UX-02**: Configuration preview before applying
- **UX-03**: Step skip options (expert mode)
- **UX-04**: Configuration reset to defaults

## Out of Scope

| Feature | Reason |
|---------|--------|
| Fresh install from live USB | Only reconfigure mode supported (run on existing NixOS) |
| GPU driver selection | Uses modesetting, assumes Intel/AMD generic |
| Custom module selection | All modules enabled by default, opinionated |
| NixOS version migration | Targets current nixos-unstable only |
| Non-UEFI systems | UEFI-only for v1 |
| Interactive partitioning UI | Use Disko for declarative partitioning |
| Remote installation via SSH | Defer to nixos-anywhere integration (v2) |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| TUI-01 | Phase 1 | Pending |
| TUI-02 | Phase 1 | Pending |
| TUI-03 | Phase 1 | Pending |
| TUI-04 | Phase 1 | Pending |
| TUI-05 | Phase 1 | Pending |
| TUI-06 | Phase 1 | Pending |
| TUI-07 | Phase 1 | Pending |
| TUI-08 | Phase 1 | Pending |
| INP-01 | Phase 2 | Pending |
| INP-02 | Phase 2 | Pending |
| INP-03 | Phase 2 | Pending |
| INP-04 | Phase 2 | Pending |
| INP-05 | Phase 2 | Pending |
| INP-06 | Phase 2 | Pending |
| INP-07 | Phase 2 | Pending |
| INP-08 | Phase 2 | Pending |
| INP-09 | Phase 2 | Pending |
| INP-10 | Phase 2 | Pending |
| CFG-01 | Phase 3 | Pending |
| CFG-02 | Phase 3 | Pending |
| CFG-03 | Phase 3 | Pending |
| CFG-04 | Phase 3 | Pending |
| CFG-05 | Phase 3 | Pending |
| CFG-06 | Phase 3 | Pending |
| CFG-07 | Phase 3 | Pending |
| CFG-08 | Phase 3 | Pending |
| CFG-09 | Phase 3 | Pending |
| EXEC-01 | Phase 4 | Pending |
| EXEC-02 | Phase 4 | Pending |
| EXEC-03 | Phase 4 | Pending |
| EXEC-04 | Phase 4 | Pending |
| EXEC-05 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 25 total
- Mapped to phases: 25
- Unmapped: 0 ✓

---

*Requirements defined: 2026-01-31*
*Last updated: 2026-01-31 after initial definition*
