# Architecture

**Analysis Date:** 2026-01-31

## Pattern Overview

**Overall:** Modular NixOS Configuration with Flake-based Inputs

**Key Characteristics:**
1. **Flake-driven inputs**: All external dependencies (nixpkgs, home-manager, niri, noctalia) defined as flake inputs
2. **Module composition**: Configuration broken into composable Nix modules by functional area
3. **Dual-layer config**: System-level (NixOS) + User-level (Home Manager) configurations
4. **Dotfile separation**: Raw configuration files (KDL, TOML, CSS) stored separately from Nix modules
5. **Host abstraction**: Single host currently ("nixos"), structured to allow multi-host expansion

## Layers

### Layer 1: Flake Inputs

**Purpose:** Define and lock external dependencies

**Location:** `flake.nix` (lines 4-21)

**Contains:**
- `nixpkgs`: nixos-unstable channel
- `home-manager`: User environment management
- `niri`: Wayland compositor
- `noctalia`: Lock screen/session manager

**Depends on:** Nix daemon, internet access for fetching

**Used by:** All downstream modules via `specialArgs`

### Layer 2: Host Configuration

**Purpose:** Machine-specific system settings

**Location:** `hosts/nixos/configuration.nix`

**Contains:**
- Nix settings (experimental features, GC, trusted users)
- Environment variables (Wayland, XDG)
- Unfree package allowance
- State version

**Depends on:** Flake inputs, hardware config

**Used by:** All system modules

### Layer 3: System Modules

**Purpose:** System-level configuration by functional area

**Location:** `modules/<category>/*.nix`

**Categories:**
- `system/`: Core OS (boot, hardware, locale, networking, security, ssh, users)
- `desktop/`: UI stack (audio, fonts, niri, wayland)
- `development/`: Dev tools (docker, editors, tools)
- `applications/`: User apps (browsers, terminals, utilities)

**Depends on:** Host config, nixpkgs

**Used by:** flake.nix module imports

**Example flow:**
```
flake.nix
  → hosts/nixos/configuration.nix
  → modules/system/networking.nix
    → enables NetworkManager
    → configures firewall
    → opens dev ports (3000-3999, 8000-8999)
```

### Layer 4: Home Manager (User Layer)

**Purpose:** User-specific configuration

**Location:** `home/milgraph.nix`

**Contains:**
- Imports home modules (shell, desktop, development)
- User identity (username, home directory, state version)
- Package list (empty - packages in system modules)
- Home Manager self-management

**Depends on:** flake.nix inputs (via `extraSpecialArgs`)

**Used by:** flake.nix `home-manager.users` mapping

**Module structure:**
```
home/milgraph.nix
  → home/modules/shell/zsh.nix
  → home/modules/shell/noctalia.nix
  → home/modules/desktop/alacritty.nix
  → home/modules/desktop/fuzzel.nix
  → home/modules/desktop/gtk.nix
  → home/modules/desktop/niri.nix
  → home/modules/development/git.nix
  → home/modules/development/editors.nix
  → home/modules/development/node.nix
```

### Layer 5: Dotfiles

**Purpose:** Raw application configuration files

**Location:** `home/dotfiles/<app>/*`

**Formats:**
- KDL (Niri config)
- INI (Fuzzel)
- CSS (GTK)
- TOML (Helix)
- JSON (OpenCode)

**Managed via:** Home Manager `xdg.configFile` or `programs.*`

**Example locations:**
- `home/dotfiles/niri/config.kdl` - Niri keybindings, layout, window rules
- `home/dotfiles/fuzzel/fuzzel.ini` - Launcher settings
- `home/dotfiles/gtk/gtk-3.0.css` - Theme colors
- `home/dotfiles/opencode/` - Complex IDE config with workflows

## Data Flow

### Rebuild Flow (System)

1. **User runs:** `sudo nixos-rebuild switch --flake .#nixos`
2. **Nix evaluates:** `flake.nix` → `nixosConfigurations.nixos`
3. **Imports modules:**
   - Host config (`hosts/nixos/configuration.nix`)
   - Hardware config (`hosts/nixos/hardware-configuration.nix`)
   - System modules (`modules/system/*.nix`)
   - Desktop modules (`modules/desktop/*.nix`)
   - Development modules (`modules/development/*.nix`)
   - Application modules (`modules/applications/*.nix`)
   - Home Manager module
4. **Home Manager activates** for user `milgraph`
5. **Generates** system profile in `/nix/var/nix/profiles/system`
6. **Switches** to new generation

### Rebuild Flow (Home Manager only)

1. **User runs:** `home-manager switch --flake .#milgraph`
2. **Evaluates:** `home/milgraph.nix` and imports
3. **Applies** user configuration
4. **Creates** home profile in `~/.local/state/home-manager`

### Configuration Inheritance

```
flake.nix (inputs)
    ↓
hosts/nixos/configuration.nix (host settings)
    ↓
modules/*/*.nix (functional modules)
    ↓
home/milgraph.nix (user settings)
    ↓
home/modules/*/*.nix (user modules)
    ↓
home/dotfiles/* (raw config files)
```

## Key Abstractions

### Module Pattern

All modules follow this structure:

```nix
{ config, pkgs, ... }:

{
  # Option declarations and assignments
  option = value;
  
  # Package installations
  environment.systemPackages = with pkgs; [ ... ];
  
  # Service definitions
  services.serviceName.enable = true;
}
```

### Home Manager Module Pattern

```nix
{ config, pkgs, ... }:

{
  programs.programName = {
    enable = true;
    settings = { ... };
  };
}
```

### Dotfile Inclusion

Niri config includes Noctalia-specific config:
```kdl
include "./noctalia.kdl"
```

## Entry Points

### System Rebuild

**Location:** `flake.nix` → `nixosConfigurations.nixos`

**Triggers:** `sudo nixos-rebuild switch --flake .#nixos`

**Responsibilities:**
- Import all modules
- Build system profile
- Activate configuration
- Update bootloader

### Home Manager Rebuild

**Location:** `flake.nix` → `home-manager.users.milgraph`

**Triggers:** `home-manager switch --flake .#milgraph`

**Responsibilities:**
- Import home modules
- Create user config files
- Link dotfiles
- Install user packages

## Error Handling

**Strategy:** Nix module system with lazy evaluation

**Patterns:**
- Optional modules via `mkIf` guards
- Conditional imports based on `enable` options
- Graceful fallback for missing hardware features

**Build-time errors:**
- Logged to `build-errors.log`
- Traced via `--show-trace` flag

## Cross-Cutting Concerns

**Logging:**
- Build logs: `build-errors.log`, `build-result.log`
- System: `journalctl`
- User: `journalctl --user`

**Validation:**
- Niri: `niri validate ~/.config/niri/config.kdl`
- Nix: `nix eval` or dry-build `--dry-run`

**Authentication:**
- SSH: Key-based only (password auth commented out)
- Polkit: Wheel group has some admin privileges

**Updates:**
- All inputs via `nix flake update`
- Per-input via `nix flake lock --update-input <name>`

---

*Architecture analysis: 2026-01-31*
