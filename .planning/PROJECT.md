# milos_niri - Reproducible NixOS Installer

## What This Is

A modular NixOS + Home Manager configuration with a TUI installer that guides users through host-specific setup. Users run the installer on their existing NixOS system, and it collects hostname, username, git credentials, locale, and paths — then generates a fully configured system identical to this one.

## Core Value

Users can reproduce this exact desktop environment (Niri compositor + Noctalia shell + dev tools) on any NixOS machine in under 10 minutes through an interactive guided installer.

## Requirements

### Validated

These are the existing capabilities of the configuration:

- ✓ **Niri compositor** — Tiling Wayland compositor with custom keybindings
- ✓ **Noctalia shell** — Desktop environment with dynamic wallpaper colors, bar, dock, notifications
- ✓ **Modular flake structure** — Separated into system modules, desktop modules, home-manager modules
- ✓ **Core packages** — alacritty, helix, neovim, brave, discord, vscode, opencode, zed
- ✓ **Audio** — PipeWire with PulseAudio compatibility
- ✓ **Development tools** — git, nodejs_24, wl-clipboard, xdg-utils
- ✓ **System services** — NetworkManager, seatd, polkit, power-profiles-daemon, upower

### Active

- [ ] **TUI Installer** — Interactive wizard collecting user-specific values
- [ ] **Host config generator** — Substitutes hardcoded values with user inputs
- [ ] **Hardware auto-detection** — Handles partition UUIDs automatically
- [ ] **Git credential setup** — Configures user.name and user.email
- [ ] **Wallpaper/avatar path handling** — Optional custom paths or defaults

### Out of Scope

- **Installation from live USB** — Only reconfigure mode supported (run on existing NixOS)
- **GPU driver selection** — Uses modesetting, assumes Intel/AMD generic
- **Custom module selection** — All modules enabled by default
- **NixOS version migration** — Targets current nixos-unstable only

## Context

**Existing codebase structure:**
- `hosts/nixos/` — System-level configuration (boot, networking, users, locale)
- `modules/` — Modular NixOS modules (system, desktop, development, applications)
- `home/` — Home Manager configuration (shell, desktop, development)
- `home/dotfiles/` — Application config files (niri, helix, fuzzel, opencode)

**Known issues to address:**
- Build fails due to helix dotfiles path reference (needs fix)
- SSH password authentication still enabled (security TODO)
- Deprecated Git/Zsh options generating warnings

**Hardcoded values requiring substitution:**
- Username, full name, git name, git email
- Hostname, timezone, keyboard layout, locale
- Wallpaper directory, avatar path, screenshot path
- Hardware UUIDs (auto-detected on new install)

## Constraints

- **[Platform]:** NixOS only (nixos-unstable channel)
- **[User Management]:** Single user setup (no multi-user support in v1)
- **[Desktop Environment]:** Niri + Noctalia only (no Hyprland/i3 support)
- **[Hardware]:** x86_64-linux, UEFI systems
- **[Installer Mode]:** Reconfigure only (no fresh install from live USB)

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| TUI installer over GUI | Matches NixOS terminal culture, works over SSH | — Pending |
| Reconfigure existing system | Simpler than full installer, leverages nixos-rebuild | — Pending |
| All modules enabled by default | Reduces complexity, opinionated setup | — Pending |
| Hardware UUIDs auto-detected | Prevents boot failures, nixos-generate-config handles | — Pending |
| Optional avatar/wallpaper paths | Not critical, users can configure later | — Pending |

---

*Last updated: 2026-01-31 after initialization*
