# Codebase Structure

**Analysis Date:** 2026-01-31

## Directory Layout

```
/home/milgraph/Projects/milos_niri/
├── flake.nix                    # Main flake entry point with inputs and outputs
├── flake.lock                   # Locked dependencies (auto-generated)
├── README.md                    # User-facing documentation
├── .gitignore                   # Git ignore rules
├── build-errors.log             # Build error log
├── build-result.log             # Build result log
├── hosts/                       # Host-specific configurations
│   └── nixos/
│       ├── configuration.nix    # Host NixOS configuration
│       └── hardware-configuration.nix  # Hardware-specific (not in git)
├── modules/                     # System-level Nix modules
│   ├── system/                  # Core system configuration
│   │   ├── boot.nix             # Bootloader and kernel settings
│   │   ├── hardware.nix         # Hardware support (bluetooth, printing, etc.)
│   │   ├── locale.nix           # Locale and timezone
│   │   ├── networking.nix       # Network and firewall settings
│   │   ├── security.nix         # Polkit, seatd, power management
│   │   ├── ssh.nix              # SSH daemon configuration
│   │   └── users.nix            # User accounts and groups
│   ├── desktop/                 # Desktop environment modules
│   │   ├── audio.nix            # PipeWire audio configuration
│   │   ├── fonts.nix            # Font packages
│   │   ├── niri.nix             # Niri compositor system packages
│   │   └── wayland.nix          # Wayland and Xwayland support
│   ├── development/             # Development tool modules
│   │   ├── docker.nix           # Docker configuration
│   │   ├── editors.nix          # Editor configurations
│   │   └── tools.nix            # Development tools (git, nodejs, etc.)
│   └── applications/            # Application packages
│       ├── browsers.nix         # Web browsers
│       ├── terminals.nix        # Terminal emulators
│       └── utilities.nix        # Utility applications
└── home/                        # Home Manager user configuration
    ├── milgraph.nix             # Home Manager entry point
    └── modules/                 # Home Manager modules
        ├── shell/               # Shell configuration
        │   ├── zsh.nix          # Zsh configuration
        │   └── noctalia.nix     # Noctalia shell integration
        ├── desktop/             # Desktop user configurations
        │   ├── alacritty.nix    # Alacritty terminal config
        │   ├── fuzzel.nix       # Fuzzel launcher config
        │   ├── gtk.nix          # GTK theme configuration
        │   └── niri.nix         # Niri user config
        └── development/         # Development user configs
            ├── editors.nix      # Editor user settings
            ├── git.nix          # Git configuration
            └── node.nix         # Node.js configuration
    └── dotfiles/                # Raw dotfile configurations
        ├── niri/                # Niri config.kdl, noctalia.kdl
        ├── fuzzel/              # Fuzzel config and themes
        ├── gtk/                 # GTK CSS customizations
        ├── helix/               # Helix editor theme
        └── opencode/            # OpenCode configuration (complex)
```

## Directory Purposes

### `/hosts/nixos/`

**Purpose:** Host-specific system configuration

**Contains:**
- `configuration.nix`: Main host configuration with Nix settings, environment variables
- `hardware-configuration.nix`: Auto-generated hardware configuration (NOT committed to git)

**Key files:**
- `configuration.nix`: Sets hostname, enables flakes, configures Nix garbage collection, sets Wayland environment variables

### `/modules/system/`

**Purpose:** Core system-level configuration

**Contains:**
- `boot.nix`: Bootloader and kernel settings
- `hardware.nix`: Bluetooth, printing, scanning, graphics support
- `locale.nix`: Timezone and locale settings
- `networking.nix`: NetworkManager, firewall with dev port ranges
- `security.nix`: Polkit, seatd, power management
- `ssh.nix`: SSH daemon with key-based auth
- `users.nix`: User account "milgraph" with groups

### `/modules/desktop/`

**Purpose:** Desktop environment and compositor configuration

**Contains:**
- `audio.nix`: PipeWire configuration
- `fonts.nix`: JetBrains Mono Nerd Font
- `niri.nix`: Niri compositor package
- `wayland.nix`: Xwayland, libinput, XDG portals

### `/modules/development/`

**Purpose:** Development environment tools

**Contains:**
- `docker.nix`: Docker with rootless support
- `editors.nix`: Neovim configuration
- `tools.nix`: Git, wget, Node.js 24, xdg-utils, wl-clipboard

### `/modules/applications/`

**Purpose:** User applications

**Contains:**
- `browsers.nix`: Brave browser
- `terminals.nix`: Alacritty terminal
- `utilities.nix`: Rofi, fuzzel, swaybg, mako, discord, thunderbird, zoom, grim, thunar, vscode, claude-code, opencode

### `/home/`

**Purpose:** Home Manager user configuration

**Contains:**
- `milgraph.nix`: Entry point importing all home modules
- `modules/`: Home Manager modules (shell, desktop, development)
- `dotfiles/`: Raw configuration files for applications

## Key File Locations

**Entry Points:**
- `flake.nix`: Main flake configuration, imports all modules
- `hosts/nixos/configuration.nix`: Host-specific NixOS config
- `home/milgraph.nix`: Home Manager entry point

**Configuration:**
- `modules/system/*.nix`: System-level config modules
- `modules/desktop/*.nix`: Desktop environment modules
- `modules/development/*.nix`: Development tool modules
- `modules/applications/*.nix`: Application packages

**User Config:**
- `home/modules/shell/zsh.nix`: Zsh shell configuration
- `home/modules/development/git.nix`: Git user configuration
- `home/modules/desktop/*.nix`: Desktop application configs

**Dotfiles:**
- `home/dotfiles/niri/config.kdl`: Niri compositor configuration (KDL format)
- `home/dotfiles/fuzzel/`: Fuzzel launcher configuration
- `home/dotfiles/gtk/`: GTK theme customizations
- `home/dotfiles/helix/`: Helix editor theme
- `home/dotfiles/opencode/`: Complex OpenCode configuration

## Naming Conventions

**Files:**
- Module files: `lowercase_with_underscores.nix` (e.g., `hardware.nix`, `networking.nix`)
- Host config: `configuration.nix`, `hardware-configuration.nix`
- Dotfiles: Application-specific names (`config.kdl`, `fuzzel.ini`, `gtk-3.0.css`)

**Directories:**
- Module categories: `lowercase` (e.g., `system`, `desktop`, `development`)
- User modules: `lowercase` (e.g., `shell`, `desktop`, `development`)
- Dotfile apps: Application names (e.g., `niri`, `fuzzel`, `gtk`, `helix`)

## Where to Add New Code

**New System Package:**
- Edit: `modules/applications/utilities.nix` or appropriate category
- Add to: `environment.systemPackages = with pkgs; [ ... ]`

**New User Package (Home Manager):**
- Edit: `home/milgraph.nix`
- Add to: `home.packages = with pkgs; [ ... ]`

**New System Module:**
- Create: `modules/<category>/new-module.nix`
- Add to: `flake.nix` `modules = [ ... ]` array

**New Home Manager Module:**
- Create: `home/modules/<category>/new-module.nix`
- Add to: `home/milgraph.nix` `imports = [ ... ]` array

**New Dotfile Configuration:**
- Create: `home/dotfiles/<app>/<config-file>`
- Configure via: Home Manager module or XDG configFile

**New Host:**
- Create: `hosts/<hostname>/configuration.nix`
- Add to: `flake.nix` `nixosConfigurations` with new host name

## Special Directories

**`home/dotfiles/opencode/`:**
- Purpose: OpenCode IDE configuration with superpowers, workflows, templates
- Generated: Partially (has `node_modules` from npm)
- Committed: Yes (not in .gitignore)

**`hosts/nixos/hardware-configuration.nix`:**
- Purpose: System-specific hardware config with disk UUIDs
- Generated: By `nixos-generate-config`
- Committed: NO (in .gitignore)

**`flake.lock`:**
- Purpose: Locked versions of all flake inputs
- Generated: Auto-maintained by Nix
- Committed: Yes (for reproducibility)

---

*Structure analysis: 2026-01-31*
