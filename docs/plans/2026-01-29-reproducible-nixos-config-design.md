# Reproducible NixOS Configuration Design

**Date:** 2026-01-29
**Author:** Claude & milgraph
**Status:** Approved

## Overview

Create a fully reproducible NixOS configuration using flakes and Home Manager that can be deployed to multiple machines. The configuration will manage both system-level packages/services and user-level dotfiles in a modular, declarative way.

## Goals

1. Create a modular NixOS configuration that separates concerns (system, desktop, development, applications)
2. Manage user dotfiles declaratively using Home Manager
3. Enable easy deployment to a second PC
4. Maintain current system functionality during migration
5. Use native Nix configuration where available (niri, noctalia, git, zsh, etc.)

## Current System

### System Configuration
- **Location:** `/etc/nixos/`
- **Files:** `configuration.nix`, `flake.nix`, `hardware-configuration.nix`, `noctalia.nix`
- **Compositor:** Niri (with programs.niri.enable)
- **Shell:** Noctalia shell
- **Desktop:** Wayland with Pipewire audio
- **Special:** HP-specific kernel parameters and modules

### User Configuration
- **User:** milgraph
- **Shell:** Zsh
- **Terminal:** Alacritty
- **Launcher:** Fuzzel, Rofi
- **Editors:** Neovim, Helix, OpenCode
- **Development:** Git, Node.js 24, Docker

### Dotfiles to Preserve
**Essential:**
- `~/.config/niri/` - Compositor config
- `~/.config/noctalia/` - Shell config (includes built-in lockscreen)
- `~/.config/alacritty/` - Terminal config
- `~/.config/fuzzel/` - Launcher config
- `~/.zshrc` - Shell config
- `~/.gitconfig`, `~/.config/git/` - Git config
- `~/.config/gtk-3.0/`, `~/.config/gtk-4.0/` - GTK themes

**Development:**
- `~/.config/helix/` - Editor config
- `~/.config/opencode/` - Editor config
- `~/.npmrc` - NPM config

## Architecture

### Repository Structure

```
~/nixos-config/
├── flake.nix                    # Main flake with inputs and outputs
├── flake.lock
│
├── hosts/
│   ├── nixos/                   # Current machine
│   │   ├── configuration.nix    # Host-specific config
│   │   └── hardware-configuration.nix
│   └── <other-pc>/              # Future: second PC
│       ├── configuration.nix
│       └── hardware-configuration.nix
│
├── modules/
│   ├── system/
│   │   ├── boot.nix            # Bootloader, kernel params, video drivers
│   │   ├── networking.nix      # NetworkManager, hostname
│   │   ├── locale.nix          # Time zone, locales, keymap
│   │   ├── users.nix           # User accounts and groups
│   │   └── security.nix        # Polkit, seatd, power management
│   │
│   ├── desktop/
│   │   ├── niri.nix            # Niri compositor (NixOS module)
│   │   ├── audio.nix           # Pipewire configuration
│   │   ├── wayland.nix         # Wayland/graphics/Xwayland
│   │   └── fonts.nix           # Font packages
│   │
│   ├── development/
│   │   ├── editors.nix         # Neovim
│   │   ├── tools.nix           # Git, wget, Node.js, dev utilities
│   │   └── docker.nix          # Docker virtualization
│   │
│   └── applications/
│       ├── browsers.nix        # Brave
│       ├── terminals.nix       # Alacritty
│       └── utilities.nix       # Fuzzel, rofi, discord, etc.
│
└── home/
    ├── milgraph.nix            # Main Home Manager config
    └── modules/
        ├── shell/
        │   ├── zsh.nix         # Zsh configuration
        │   └── noctalia.nix    # Noctalia shell with lockscreen
        │
        ├── desktop/
        │   ├── niri.nix        # Niri user config
        │   ├── alacritty.nix   # Terminal config
        │   ├── fuzzel.nix      # Launcher config
        │   └── gtk.nix         # GTK themes
        │
        └── development/
            ├── git.nix         # Git configuration
            ├── editors.nix     # Helix, OpenCode configs
            └── node.nix        # NPM configuration
```

### Flake Configuration

**Inputs:**
- `nixpkgs` - Main package repository (unstable)
- `home-manager` - User configuration management
- `noctalia` - Noctalia shell flake
- `niri` - Niri compositor flake (sodiboo/niri-flake)

**Outputs:**
- `nixosConfigurations.nixos` - Current machine configuration
- Future: `nixosConfigurations.<other-pc>` - Second machine

**Key Design Decisions:**
1. Home Manager as NixOS module (not standalone) - single rebuild command
2. All inputs follow nixpkgs for consistency
3. specialArgs passes inputs to all modules
4. Modular imports for easy per-host customization

### Configuration Approach

**Declarative (Native Nix):**
- Noctalia: `programs.noctalia-shell.settings = { ... }`
- Niri: `programs.niri.settings = { ... }` (with build-time validation)
- Git: `programs.git.*`
- Zsh: `programs.zsh.*`
- Alacritty: `programs.alacritty.*`
- Fuzzel: `programs.fuzzel.*`

**File-based (Copy dotfiles):**
- Helix: `home.file.".config/helix/..."`
- OpenCode: `home.file.".config/opencode/..."`
- GTK: Mix of `gtk.enable` + custom files if needed

## Module Breakdown

### System Modules

#### boot.nix
- Systemd-boot bootloader
- EFI variables
- HP-specific kernel modules: `i8042`, `hid_generic`, `usbhid`
- HP-specific kernel params: `i8042.reset`, `i8042.nomux`, `i915.enable_psr=0`
- Video drivers: `modesetting`

#### networking.nix
- Hostname: `nixos` (host-specific, override per machine)
- NetworkManager enabled
- Firewall configuration

#### locale.nix
- Time zone: `Africa/Tunis`
- Default locale: `en_US.UTF-8`
- Extra locales: `ar_TN.UTF-8` for LC_ADDRESS, LC_IDENTIFICATION, etc.
- Console keymap: `fr`
- X11 keymap: `fr`

#### users.nix
- User: `milgraph`
- Description: `Ahmed Chouaya`
- Groups: `networkmanager`, `wheel`, `audio`, `video`, `docker`, `seat`, `input`
- Shell: `pkgs.zsh`
- Enable zsh globally: `programs.zsh.enable = true`

#### security.nix
- Polkit enabled
- Seatd enabled
- Power profiles daemon
- UPower service

### Desktop Modules

#### niri.nix
- `programs.niri.enable = true` (NixOS module)
- Niri package
- Xwayland-satellite

#### audio.nix
- Pipewire enabled
- PulseAudio support
- ALSA support

#### wayland.nix
- Xserver enabled (for Xwayland)
- Libinput enabled
- Wayland-related packages: libinput, libdrm, libxkbcommon, pixman, etc.
- Build tools: meson, ninja
- Wayland libraries: wayland-protocols, libdisplay-info, libliftoff, hwdata

#### fonts.nix
- JetBrains Mono Nerd Font
- Default font packages

### Development Modules

#### editors.nix
- Neovim

#### tools.nix
- Git
- Wget
- Node.js 24
- XDG utils
- Wl-clipboard

#### docker.nix
- `virtualisation.docker.enable = true`

### Application Modules

#### browsers.nix
- Brave browser

#### terminals.nix
- Alacritty

#### utilities.nix
- Launchers: fuzzel, rofi
- Background: swaybg
- Notifications: mako
- Idle manager: swayidle (note: swaylock removed - noctalia has built-in lockscreen)
- Communication: discord, thunderbird, zoom-us
- Recording: gpu-screen-recorder, gpu-screen-recorder-gtk
- Development: claude-code, opencode, vscode
- Misc: glibc, pcre2, seatd, phinger-cursors

### Home Manager Modules

#### shell/zsh.nix
- Extract configuration from `~/.zshrc`
- Use `programs.zsh.*` for declarative config
- History, aliases, environment variables
- Plugins/completions if any

#### shell/noctalia.nix
- Import: `inputs.noctalia.homeModules.default`
- `programs.noctalia-shell.enable = true`
- Extract settings from `~/.config/noctalia/`
- Lockscreen configuration:
  - `lockOnSuspend`
  - `showSessionButtonsOnLockScreen`
  - `compactLockScreen`
  - `enableLockScreenCountdown`
  - `lockScreenCountdownDuration`
- Optional: `programs.noctalia-shell.systemd.enable = true`
- Swayidle integration for auto-lock

#### desktop/niri.nix
- Import: `inputs.niri.homeModules.config`
- `programs.niri.settings = { ... }`
- Extract from `~/.config/niri/config.kdl`
- Build-time validation via niri-flake
- Output settings (scale, position, etc.)
- Keybinds, window rules, etc.

#### desktop/alacritty.nix
- `programs.alacritty.enable = true`
- Extract from `~/.config/alacritty/alacritty.toml`
- Font, colors, window settings

#### desktop/fuzzel.nix
- `programs.fuzzel.enable = true` (if available)
- Or `home.file.".config/fuzzel/fuzzel.ini".source = ...`
- Extract from `~/.config/fuzzel/`

#### desktop/gtk.nix
- `gtk.enable = true`
- GTK theme, icon theme
- Cursor theme (phinger-cursors)
- Extract from `~/.config/gtk-3.0/settings.ini` and `~/.config/gtk-4.0/settings.ini`

#### development/git.nix
- `programs.git.enable = true`
- Extract from `~/.gitconfig` and `~/.config/git/config`
- User name, email
- Aliases, core settings
- **Important:** Don't commit tokens/credentials

#### development/editors.nix
- Helix: `home.file.".config/helix/config.toml".source = ...`
- OpenCode: `home.file.".config/opencode/settings.json".source = ...`
- Copy theme files if present

#### development/node.nix
- Extract from `~/.npmrc`
- **Important:** Don't commit auth tokens
- Use `home.file.".npmrc".text = ...` or `programs.npm.*` if available

## Migration Process

### Phase 1: Repository Setup
1. Create `~/nixos-config/` directory
2. Initialize git repository
3. Create directory structure (hosts/, modules/, home/)
4. Copy `/etc/nixos/hardware-configuration.nix` to `hosts/nixos/`
5. Create initial `flake.nix` with all inputs
6. Create `flake.lock` with `nix flake update`

### Phase 2: Extract System Configuration
1. Create all system modules in `modules/system/`
2. Create all desktop modules in `modules/desktop/`
3. Create all development modules in `modules/development/`
4. Create all application modules in `modules/applications/`
5. Create host-specific `hosts/nixos/configuration.nix`
6. Import all modules in `flake.nix`

### Phase 3: Extract User Configurations
**Priority 1 - Shell:**
1. Extract `~/.zshrc` → `home/modules/shell/zsh.nix`
2. Extract `~/.config/noctalia/` → `home/modules/shell/noctalia.nix`

**Priority 2 - Desktop:**
3. Extract `~/.config/niri/config.kdl` → `home/modules/desktop/niri.nix`
4. Extract `~/.config/alacritty/alacritty.toml` → `home/modules/desktop/alacritty.nix`
5. Extract `~/.config/fuzzel/` → `home/modules/desktop/fuzzel.nix`
6. Extract GTK settings → `home/modules/desktop/gtk.nix`

**Priority 3 - Development:**
7. Extract `~/.gitconfig` → `home/modules/development/git.nix`
8. Copy `~/.config/helix/` → `home/modules/development/editors.nix`
9. Copy `~/.config/opencode/` → `home/modules/development/editors.nix`
10. Extract `~/.npmrc` → `home/modules/development/node.nix`

### Phase 4: Build & Test
1. Test build: `nixos-rebuild build --flake ~/nixos-config#nixos`
2. Check for errors, fix module issues
3. Optional: Build VM: `nixos-rebuild build-vm --flake ~/nixos-config#nixos`
4. Test VM to verify configuration
5. Switch to new config: `sudo nixos-rebuild switch --flake ~/nixos-config#nixos`

### Phase 5: Validation
**Validation Checklist:**
- [ ] System boots successfully
- [ ] Niri compositor starts
- [ ] Noctalia shell loads with correct settings
- [ ] Noctalia lockscreen works (test with Super+L or configured keybind)
- [ ] Alacritty opens with correct theme/font
- [ ] Fuzzel launcher works
- [ ] Audio (Pipewire) functional
- [ ] Network connectivity maintained
- [ ] Git config correct (name, email, aliases)
- [ ] Helix and OpenCode configs preserved
- [ ] Docker accessible
- [ ] All keybinds functional

### Phase 6: Version Control
1. Commit all files: `git add . && git commit -m "Initial reproducible NixOS config"`
2. Push to remote repository (GitHub/GitLab recommended)
3. Tag version: `git tag v1.0.0`

### Phase 7: Deploy to Other PC
1. Boot second PC with NixOS installer
2. Clone repository: `git clone <repo-url> /mnt/etc/nixos`
3. Generate hardware config: `nixos-generate-config --root /mnt`
4. Copy `/mnt/etc/nixos/hardware-configuration.nix` to `hosts/<other-pc>/`
5. Create `hosts/<other-pc>/configuration.nix` with machine-specific settings
6. Update `flake.nix` to add new host configuration
7. Install: `nixos-install --flake /mnt/etc/nixos#<other-pc>`

## Testing & Safety

### Incremental Testing
- Build after each module: `nixos-rebuild build --flake ~/nixos-config#nixos`
- Check evaluation errors immediately
- Test individual module changes before combining

### Backup & Rollback
**Safety measures:**
1. Keep `/etc/nixos/` untouched until new config proven
2. NixOS generations available in bootloader
3. Git history for reverting changes
4. Rollback command: `sudo nixos-rebuild switch --rollback`

**If something breaks:**
1. Reboot and select previous generation from bootloader menu
2. Or: `sudo nixos-rebuild switch` from `/etc/nixos/` (old config)
3. Debug: `nixos-rebuild build --flake ~/nixos-config#nixos --show-trace`

### Common Gotchas
1. **Absolute paths** - Use relative paths or Nix variables
2. **Secrets** - Don't commit tokens in `.gitconfig`, `.npmrc`, etc.
3. **Hardware-specific** - Monitor names, device paths differ between machines
4. **Version drift** - Using unstable means configs may need updates
5. **Home Manager conflicts** - Can't manage same file with `programs.*` and `home.file`

## Implementation Checklist

- [ ] Phase 1: Repository setup
- [ ] Phase 2: System module extraction
- [ ] Phase 3: Home Manager module extraction
- [ ] Phase 4: Build and test
- [ ] Phase 5: Validate all functionality
- [ ] Phase 6: Commit and push to git
- [ ] Phase 7: Deploy to second PC (when ready)

## Benefits

1. **Reproducibility** - Exact same configuration on multiple machines
2. **Version control** - Track all changes, easy rollback
3. **Modularity** - Enable/disable features per machine
4. **Declarative** - Configuration as code, no manual setup
5. **Build-time validation** - Catch errors before deployment (especially with niri-flake)
6. **Single command updates** - `nixos-rebuild switch --flake ~/nixos-config#nixos`

## Future Enhancements

1. **Secrets management** - Use sops-nix or agenix for sensitive data
2. **Per-machine modules** - Create machine-specific feature modules
3. **Shared home configs** - Multiple users with shared base config
4. **CI/CD** - GitHub Actions to validate builds on push
5. **NixOS anywhere** - Remote deployment tooling

## References

- [NixOS Manual](https://nixos.org/manual/nixos/stable/)
- [Home Manager Manual](https://nix-community.github.io/home-manager/)
- [Noctalia NixOS Docs](https://docs.noctalia.dev/getting-started/nixos/)
- [Niri Flake](https://github.com/sodiboo/niri-flake)
- [Niri Official Docs](https://github.com/YaLTeR/niri)
