# Requirements: milos_niri - NixOS TUI Installer

**Defined:** 2026-01-31
**Updated:** 2026-01-31 (Phase 2 complete)
**Core Value:** Users can reproduce this exact desktop environment (Niri compositor + Noctalia shell + dev tools) on any NixOS machine in under 10 minutes through an interactive guided installer.

## v1 Requirements

### TUI Framework

- [x] **TUI-01**: Initialize Ratatui project structure with Cargo ✓ (Phase 1)
- [x] **TUI-02**: Set up Crossterm event loop for keyboard input ✓ (Phase 1)
- [x] **TUI-03**: Implement centralized state management store ✓ (Phase 1)
- [x] **TUI-04**: Create wizard flow with steps and navigation ✓ (Phase 1)
- [x] **TUI-05**: Add color-eyre error handling with beautiful backtraces ✓ (Phase 1)
- [x] **TUI-06**: Display MILOS pixel art logo on installer startup screen ✓ (Phase 1)
- [x] **TUI-07**: Pixel art logo with color scheme matching Noctalia theme ✓ (Phase 1)
- [x] **TUI-08**: Logo animation effect (optional polish) ✓ (Phase 1)

### Input Collection

- [x] **INP-01**: Hostname input with validation (alphanumeric, no spaces) ✓ (Phase 2)
- [x] **INP-02**: Username input with validation (lowercase, alphanumeric) ✓ (Phase 2)
- [x] **INP-03**: Full name input (free text) ✓ (Phase 2)
- [x] **INP-04**: Git username input (for commit author) ✓ (Phase 2)
- [x] **INP-05**: Git email input (for commit author) ✓ (Phase 2)
- [x] **INP-06**: Timezone selection (region/city list) ✓ (Phase 2)
- [x] **INP-07**: Keyboard layout selection (common layouts with type-to-filter) ✓ (Phase 2)
- [x] **INP-08**: Wallpaper directory path (default: ~/Pictures/Wallpapers) ✓ (Phase 2)
- [x] **INP-09**: Avatar image path (optional, default: none) ✓ (Phase 2)
- [x] **INP-10**: Screenshot path (default: ~/Pictures/Screenshots) ✓ (Phase 2)

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

## Traceability Matrix

| Phase | Requirements | Status |
|-------|--------------|--------|
| 1 - TUI Foundation | TUI-01 through TUI-08 | 8/8 Complete |
| 2 - Input Collection | INP-01 through INP-10 | 10/10 Complete |
| 3 - Config Generation | CFG-01 through CFG-09 | 0/9 Pending |
| 4 - Execution | EXEC-01 through EXEC-05 | 0/5 Pending |

**Overall Progress:** 18/32 requirements complete (56%)

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

---

*Requirements defined: 2026-01-31*
*Last updated: 2026-01-31 after Phase 2 completion*
