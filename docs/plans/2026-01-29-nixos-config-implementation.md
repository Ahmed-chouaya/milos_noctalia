# Reproducible NixOS Configuration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a modular, reproducible NixOS configuration with Home Manager that manages system packages, services, and user dotfiles declaratively.

**Architecture:** Flake-based NixOS configuration with modular system/desktop/development/application modules. Home Manager integrated as NixOS module managing user dotfiles. Uses native Nix configuration for niri, noctalia, git, zsh, alacritty, and fuzzel.

**Tech Stack:** NixOS (unstable), Home Manager, Niri compositor, Noctalia shell, niri-flake, Nix flakes

---

## Task 1: Repository Structure Setup

**Files:**
- Create: `hosts/nixos/.gitkeep`
- Create: `modules/system/.gitkeep`
- Create: `modules/desktop/.gitkeep`
- Create: `modules/development/.gitkeep`
- Create: `modules/applications/.gitkeep`
- Create: `home/modules/shell/.gitkeep`
- Create: `home/modules/desktop/.gitkeep`
- Create: `home/modules/development/.gitkeep`
- Create: `.gitignore`

**Step 1: Create directory structure**

Run:
```bash
mkdir -p hosts/nixos
mkdir -p modules/{system,desktop,development,applications}
mkdir -p home/modules/{shell,desktop,development}
```

Expected: Directories created successfully

**Step 2: Create .gitkeep files**

Run:
```bash
touch hosts/nixos/.gitkeep
touch modules/system/.gitkeep
touch modules/desktop/.gitkeep
touch modules/development/.gitkeep
touch modules/applications/.gitkeep
touch home/modules/shell/.gitkeep
touch home/modules/desktop/.gitkeep
touch home/modules/development/.gitkeep
```

Expected: Empty .gitkeep files created

**Step 3: Create .gitignore**

Create file `.gitignore`:
```gitignore
# Build results
result
result-*

# Nix build artifacts
*.qcow2

# Secrets (if added later)
secrets/
*.key
*.pem

# Editor files
.vscode/
.idea/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db
```

**Step 4: Initialize git repository**

Run:
```bash
git init
git add .
git commit -m "chore: initialize repository structure"
```

Expected: Git repository initialized with directory structure committed

---

## Task 2: Copy Hardware Configuration

**Files:**
- Create: `hosts/nixos/hardware-configuration.nix`

**Step 1: Copy hardware configuration from /etc/nixos**

Run:
```bash
sudo cp /etc/nixos/hardware-configuration.nix hosts/nixos/hardware-configuration.nix
sudo chown milgraph:users hosts/nixos/hardware-configuration.nix
```

Expected: hardware-configuration.nix copied successfully

**Step 2: Verify the file**

Run:
```bash
cat hosts/nixos/hardware-configuration.nix | head -20
```

Expected: File contains hardware scan results with boot.initrd, fileSystems, etc.

**Step 3: Commit**

Run:
```bash
git add hosts/nixos/hardware-configuration.nix
git commit -m "feat: add hardware configuration for nixos host"
```

---

## Task 3: Create Main Flake

**Files:**
- Create: `flake.nix`

**Step 1: Create flake.nix with inputs**

Create file `flake.nix`:
```nix
{
  description = "Modular NixOS configuration with Home Manager";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    noctalia = {
      url = "github:noctalia-dev/noctalia-shell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    niri = {
      url = "github:sodiboo/niri-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, home-manager, noctalia, niri, ... }: {
    nixosConfigurations = {
      nixos = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; };
        modules = [
          # Host-specific configuration
          ./hosts/nixos/configuration.nix
          ./hosts/nixos/hardware-configuration.nix

          # System modules
          ./modules/system/boot.nix
          ./modules/system/networking.nix
          ./modules/system/locale.nix
          ./modules/system/users.nix
          ./modules/system/security.nix

          # Desktop environment
          ./modules/desktop/niri.nix
          ./modules/desktop/audio.nix
          ./modules/desktop/wayland.nix
          ./modules/desktop/fonts.nix

          # Development
          ./modules/development/editors.nix
          ./modules/development/tools.nix
          ./modules/development/docker.nix

          # Applications
          ./modules/applications/browsers.nix
          ./modules/applications/terminals.nix
          ./modules/applications/utilities.nix

          # Home Manager as NixOS module
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.extraSpecialArgs = { inherit inputs; };
            home-manager.users.milgraph = import ./home/milgraph.nix;
          }
        ];
      };
    };
  };
}
```

**Step 2: Generate flake.lock**

Run:
```bash
nix flake update
```

Expected: flake.lock created with locked versions of all inputs

**Step 3: Commit**

Run:
```bash
git add flake.nix flake.lock
git commit -m "feat: add main flake with all inputs and module imports"
```

---

## Task 4: Create System Modules - Boot

**Files:**
- Create: `modules/system/boot.nix`

**Step 1: Create boot.nix**

Create file `modules/system/boot.nix`:
```nix
{ config, pkgs, ... }:

{
  # Bootloader configuration
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # HP-specific kernel modules for input devices
  boot.kernelModules = [ "i8042" "hid_generic" "usbhid" ];

  # HP-specific kernel parameters for stability
  boot.kernelParams = [
    "i8042.reset"      # Reset keyboard controller
    "i8042.nomux"      # Disable i8042 multiplexing
    "i915.enable_psr=0" # Disable Intel PSR (panel self refresh)
  ];

  # Video drivers
  services.xserver.videoDrivers = [ "modesetting" ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/system/boot.nix
git commit -m "feat(system): add boot configuration with HP-specific settings"
```

---

## Task 5: Create System Modules - Networking

**Files:**
- Create: `modules/system/networking.nix`

**Step 1: Create networking.nix**

Create file `modules/system/networking.nix`:
```nix
{ config, pkgs, ... }:

{
  # Hostname (can be overridden per host)
  networking.hostName = "nixos";

  # Enable NetworkManager for network management
  networking.networkmanager.enable = true;

  # Firewall configuration
  # networking.firewall.allowedTCPPorts = [ ];
  # networking.firewall.allowedUDPPorts = [ ];
  # Or disable the firewall altogether:
  # networking.firewall.enable = false;
}
```

**Step 2: Commit**

Run:
```bash
git add modules/system/networking.nix
git commit -m "feat(system): add networking configuration"
```

---

## Task 6: Create System Modules - Locale

**Files:**
- Create: `modules/system/locale.nix`

**Step 1: Create locale.nix**

Create file `modules/system/locale.nix`:
```nix
{ config, pkgs, ... }:

{
  # Time zone
  time.timeZone = "Africa/Tunis";

  # Locale settings
  i18n.defaultLocale = "en_US.UTF-8";

  i18n.extraLocaleSettings = {
    LC_ADDRESS = "ar_TN.UTF-8";
    LC_IDENTIFICATION = "ar_TN.UTF-8";
    LC_MEASUREMENT = "ar_TN.UTF-8";
    LC_MONETARY = "ar_TN.UTF-8";
    LC_NAME = "ar_TN.UTF-8";
    LC_NUMERIC = "ar_TN.UTF-8";
    LC_PAPER = "ar_TN.UTF-8";
    LC_TELEPHONE = "ar_TN.UTF-8";
    LC_TIME = "ar_TN.UTF-8";
  };

  # Console keymap
  console.keyMap = "fr";

  # X11 keymap
  services.xserver.xkb = {
    layout = "fr";
    variant = "";
  };
}
```

**Step 2: Commit**

Run:
```bash
git add modules/system/locale.nix
git commit -m "feat(system): add locale and keyboard configuration"
```

---

## Task 7: Create System Modules - Users

**Files:**
- Create: `modules/system/users.nix`

**Step 1: Create users.nix**

Create file `modules/system/users.nix`:
```nix
{ config, pkgs, ... }:

{
  # Enable zsh system-wide
  programs.zsh.enable = true;

  # Define user account
  users.users.milgraph = {
    isNormalUser = true;
    description = "Ahmed Chouaya";
    extraGroups = [
      "networkmanager"
      "wheel"
      "audio"
      "video"
      "docker"
      "seat"
      "input"
    ];
    shell = pkgs.zsh;
    packages = with pkgs; [];
  };
}
```

**Step 2: Commit**

Run:
```bash
git add modules/system/users.nix
git commit -m "feat(system): add user configuration for milgraph"
```

---

## Task 8: Create System Modules - Security

**Files:**
- Create: `modules/system/security.nix`

**Step 1: Create security.nix**

Create file `modules/system/security.nix`:
```nix
{ config, pkgs, ... }:

{
  # Polkit for privilege escalation
  security.polkit.enable = true;

  # Seatd for seat management
  services.seatd.enable = true;

  # Power management
  services.power-profiles-daemon.enable = true;
  services.upower.enable = true;
}
```

**Step 2: Commit**

Run:
```bash
git add modules/system/security.nix
git commit -m "feat(system): add security and power management configuration"
```

---

## Task 9: Create Desktop Modules - Niri

**Files:**
- Create: `modules/desktop/niri.nix`

**Step 1: Create niri.nix**

Create file `modules/desktop/niri.nix`:
```nix
{ config, pkgs, ... }:

{
  # Enable Niri compositor
  programs.niri.enable = true;

  # Niri package and related tools
  environment.systemPackages = with pkgs; [
    niri
    xwayland-satellite
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/desktop/niri.nix
git commit -m "feat(desktop): add niri compositor configuration"
```

---

## Task 10: Create Desktop Modules - Audio

**Files:**
- Create: `modules/desktop/audio.nix`

**Step 1: Create audio.nix**

Create file `modules/desktop/audio.nix`:
```nix
{ config, pkgs, ... }:

{
  # Pipewire audio server
  services.pipewire = {
    enable = true;
    pulse.enable = true;  # PulseAudio compatibility
    alsa.enable = true;   # ALSA support
  };
}
```

**Step 2: Commit**

Run:
```bash
git add modules/desktop/audio.nix
git commit -m "feat(desktop): add pipewire audio configuration"
```

---

## Task 11: Create Desktop Modules - Wayland

**Files:**
- Create: `modules/desktop/wayland.nix`

**Step 1: Create wayland.nix**

Create file `modules/desktop/wayland.nix`:
```nix
{ config, pkgs, ... }:

{
  # Enable X server (for Xwayland support)
  services.xserver.enable = true;

  # Enable libinput for input device management
  services.libinput.enable = true;

  # Wayland-related packages
  environment.systemPackages = with pkgs; [
    # Wayland protocols and libraries
    kdePackages.wayland-protocols
    libinput
    libdrm
    libxkbcommon
    pixman

    # Build tools (may be needed for some packages)
    meson
    ninja

    # Wayland-specific libraries
    libdisplay-info
    libliftoff
    hwdata
    seatd
    pcre2
    glibc
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/desktop/wayland.nix
git commit -m "feat(desktop): add wayland and graphics configuration"
```

---

## Task 12: Create Desktop Modules - Fonts

**Files:**
- Create: `modules/desktop/fonts.nix`

**Step 1: Create fonts.nix**

Create file `modules/desktop/fonts.nix`:
```nix
{ config, pkgs, ... }:

{
  # Font packages
  fonts.packages = with pkgs; [
    nerd-fonts.jetbrains-mono
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/desktop/fonts.nix
git commit -m "feat(desktop): add font configuration"
```

---

## Task 13: Create Development Modules - Editors

**Files:**
- Create: `modules/development/editors.nix`

**Step 1: Create editors.nix**

Create file `modules/development/editors.nix`:
```nix
{ config, pkgs, ... }:

{
  # Editor packages
  environment.systemPackages = with pkgs; [
    neovim
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/development/editors.nix
git commit -m "feat(development): add editor configuration"
```

---

## Task 14: Create Development Modules - Tools

**Files:**
- Create: `modules/development/tools.nix`

**Step 1: Create tools.nix**

Create file `modules/development/tools.nix`:
```nix
{ config, pkgs, ... }:

{
  # Development tools
  environment.systemPackages = with pkgs; [
    git
    wget
    nodejs_24
    xdg-utils
    wl-clipboard
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/development/tools.nix
git commit -m "feat(development): add development tools configuration"
```

---

## Task 15: Create Development Modules - Docker

**Files:**
- Create: `modules/development/docker.nix`

**Step 1: Create docker.nix**

Create file `modules/development/docker.nix`:
```nix
{ config, pkgs, ... }:

{
  # Enable Docker
  virtualisation.docker.enable = true;
}
```

**Step 2: Commit**

Run:
```bash
git add modules/development/docker.nix
git commit -m "feat(development): add docker configuration"
```

---

## Task 16: Create Application Modules - Browsers

**Files:**
- Create: `modules/applications/browsers.nix`

**Step 1: Create browsers.nix**

Create file `modules/applications/browsers.nix`:
```nix
{ config, pkgs, ... }:

{
  # Browser packages
  environment.systemPackages = with pkgs; [
    brave
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/applications/browsers.nix
git commit -m "feat(applications): add browser configuration"
```

---

## Task 17: Create Application Modules - Terminals

**Files:**
- Create: `modules/applications/terminals.nix`

**Step 1: Create terminals.nix**

Create file `modules/applications/terminals.nix`:
```nix
{ config, pkgs, ... }:

{
  # Terminal emulators
  environment.systemPackages = with pkgs; [
    alacritty
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/applications/terminals.nix
git commit -m "feat(applications): add terminal configuration"
```

---

## Task 18: Create Application Modules - Utilities

**Files:**
- Create: `modules/applications/utilities.nix`

**Step 1: Create utilities.nix**

Create file `modules/applications/utilities.nix`:
```nix
{ config, pkgs, ... }:

{
  # Utility applications
  environment.systemPackages = with pkgs; [
    # Launchers
    rofi
    fuzzel

    # Background and notifications
    swaybg
    mako
    swayidle

    # Communication
    discord
    thunderbird
    zoom-us

    # Screen recording
    gpu-screen-recorder
    gpu-screen-recorder-gtk

    # Development tools
    claude-code
    opencode
    vscode

    # Miscellaneous
    phinger-cursors
  ];
}
```

**Step 2: Commit**

Run:
```bash
git add modules/applications/utilities.nix
git commit -m "feat(applications): add utilities configuration"
```

---

## Task 19: Create Host Configuration

**Files:**
- Create: `hosts/nixos/configuration.nix`

**Step 1: Create configuration.nix**

Create file `hosts/nixos/configuration.nix`:
```nix
{ config, pkgs, ... }:

{
  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;

  # Enable experimental features for flakes
  nix.settings.experimental-features = [ "nix-command" "flakes" ];

  # System state version
  # IMPORTANT: Don't change this after installation
  system.stateVersion = "25.11";
}
```

**Step 2: Commit**

Run:
```bash
git add hosts/nixos/configuration.nix
git commit -m "feat(hosts): add nixos host configuration"
```

---

## Task 20: Create Home Manager Main Configuration

**Files:**
- Create: `home/milgraph.nix`

**Step 1: Create milgraph.nix**

Create file `home/milgraph.nix`:
```nix
{ config, pkgs, inputs, ... }:

{
  imports = [
    # Shell configuration
    ./modules/shell/zsh.nix
    ./modules/shell/noctalia.nix

    # Desktop environment
    ./modules/desktop/niri.nix
    ./modules/desktop/alacritty.nix
    ./modules/desktop/fuzzel.nix
    ./modules/desktop/gtk.nix

    # Development tools
    ./modules/development/git.nix
    ./modules/development/editors.nix
    ./modules/development/node.nix
  ];

  home = {
    username = "milgraph";
    homeDirectory = "/home/milgraph";
    stateVersion = "25.11";

    # User-specific packages (empty for now, packages in system)
    packages = with pkgs; [];
  };

  # Let Home Manager manage itself
  programs.home-manager.enable = true;
}
```

**Step 2: Commit**

Run:
```bash
git add home/milgraph.nix
git commit -m "feat(home): add home manager main configuration"
```

---

## Task 21: Extract Zsh Configuration

**Files:**
- Create: `home/modules/shell/zsh.nix`
- Read: `~/.zshrc`

**Step 1: Read current .zshrc**

Run:
```bash
cat ~/.zshrc
```

Expected: Display current zsh configuration

**Step 2: Create zsh.nix with basic configuration**

Create file `home/modules/shell/zsh.nix`:
```nix
{ config, pkgs, ... }:

{
  programs.zsh = {
    enable = true;

    # Enable completion
    enableCompletion = true;

    # History configuration
    history = {
      size = 10000;
      path = "${config.home.homeDirectory}/.zsh_history";
    };

    # Shell aliases (extract from your .zshrc)
    shellAliases = {
      # Add your aliases here after reviewing .zshrc
      # Example:
      # ll = "ls -la";
      # update = "sudo nixos-rebuild switch --flake ~/milos_niri#nixos";
    };

    # Additional init commands (extract from your .zshrc)
    initExtra = ''
      # Add your custom zsh configuration here
      # Example: prompt configuration, functions, etc.
    '';
  };
}
```

**Step 3: Note for manual extraction**

Note: You'll need to manually review ~/.zshrc and extract:
- Aliases to shellAliases
- Functions to initExtra
- Environment variables to sessionVariables
- Plugin configurations to plugins

**Step 4: Commit**

Run:
```bash
git add home/modules/shell/zsh.nix
git commit -m "feat(home/shell): add zsh configuration (needs manual extraction)"
```

---

## Task 22: Extract Noctalia Configuration

**Files:**
- Create: `home/modules/shell/noctalia.nix`
- Read: `~/.config/noctalia/`

**Step 1: Check noctalia config directory**

Run:
```bash
ls -la ~/.config/noctalia/
```

Expected: Display noctalia configuration files

**Step 2: Create noctalia.nix**

Create file `home/modules/shell/noctalia.nix`:
```nix
{ config, pkgs, inputs, ... }:

{
  imports = [
    inputs.noctalia.homeModules.default
  ];

  programs.noctalia-shell = {
    enable = true;

    # Enable systemd service
    systemd.enable = true;

    # Configuration settings
    settings = {
      # General settings
      general = {
        lockOnSuspend = true;
        showSessionButtonsOnLockScreen = true;
        showHibernateOnLockScreen = false;
        compactLockScreen = false;
        enableLockScreenCountdown = true;
        lockScreenCountdownDuration = 10000;
      };

      # Add more settings by reading from ~/.config/noctalia/
      # You can use the "Copy Settings" feature in Noctalia's settings panel
      # to export your current configuration
    };
  };
}
```

**Step 3: Note for manual extraction**

Note: Open Noctalia Settings Panel → General → "Copy Settings" to export your current configuration.
Paste the exported settings into the settings attribute.

**Step 4: Commit**

Run:
```bash
git add home/modules/shell/noctalia.nix
git commit -m "feat(home/shell): add noctalia configuration (needs manual extraction)"
```

---

## Task 23: Extract Niri Configuration

**Files:**
- Create: `home/modules/desktop/niri.nix`
- Read: `~/.config/niri/config.kdl`

**Step 1: Read current niri config**

Run:
```bash
cat ~/.config/niri/config.kdl
```

Expected: Display current niri KDL configuration

**Step 2: Create niri.nix**

Create file `home/modules/desktop/niri.nix`:
```nix
{ config, pkgs, inputs, ... }:

{
  imports = [
    inputs.niri.homeModules.config
  ];

  programs.niri = {
    settings = {
      # Extract your niri configuration here
      # Convert from KDL to Nix attributes
      # Example structure:
      # outputs."eDP-1".scale = 2.0;
      # binds = {
      #   "Mod+Return".action.spawn = [ "alacritty" ];
      # };

      # This requires manual conversion from your config.kdl
      # The niri-flake documentation shows the attribute structure
    };
  };
}
```

**Step 3: Note for manual extraction**

Note: You need to convert your ~/.config/niri/config.kdl to Nix attributes.
Reference: https://github.com/sodiboo/niri-flake for conversion examples.
The niri-flake provides build-time validation of your config.

**Step 4: Commit**

Run:
```bash
git add home/modules/desktop/niri.nix
git commit -m "feat(home/desktop): add niri configuration (needs manual conversion)"
```

---

## Task 24: Extract Alacritty Configuration

**Files:**
- Create: `home/modules/desktop/alacritty.nix`
- Read: `~/.config/alacritty/alacritty.toml`

**Step 1: Read current alacritty config**

Run:
```bash
cat ~/.config/alacritty/alacritty.toml
```

Expected: Display current alacritty TOML configuration

**Step 2: Create alacritty.nix**

Create file `home/modules/desktop/alacritty.nix`:
```nix
{ config, pkgs, ... }:

{
  programs.alacritty = {
    enable = true;

    # Settings converted from alacritty.toml
    settings = {
      # Extract from your alacritty.toml
      # Example structure:
      # window = {
      #   padding.x = 10;
      #   padding.y = 10;
      # };
      # font = {
      #   normal.family = "JetBrainsMono Nerd Font";
      #   size = 12.0;
      # };
      # colors = {
      #   primary = {
      #     background = "#1e1e1e";
      #     foreground = "#d4d4d4";
      #   };
      # };
    };
  };
}
```

**Step 3: Note for manual extraction**

Note: Convert your ~/.config/alacritty/alacritty.toml to Nix attributes.
Home Manager's alacritty module maps directly to TOML structure.

**Step 4: Commit**

Run:
```bash
git add home/modules/desktop/alacritty.nix
git commit -m "feat(home/desktop): add alacritty configuration (needs manual conversion)"
```

---

## Task 25: Extract Fuzzel Configuration

**Files:**
- Create: `home/modules/desktop/fuzzel.nix`
- Read: `~/.config/fuzzel/`

**Step 1: Check fuzzel config**

Run:
```bash
ls -la ~/.config/fuzzel/
cat ~/.config/fuzzel/fuzzel.ini 2>/dev/null || echo "No fuzzel.ini found"
```

Expected: Display fuzzel configuration if exists

**Step 2: Create fuzzel.nix**

Create file `home/modules/desktop/fuzzel.nix`:
```nix
{ config, pkgs, ... }:

{
  programs.fuzzel = {
    enable = true;

    # Settings from fuzzel.ini (if exists)
    settings = {
      # Extract from ~/.config/fuzzel/fuzzel.ini
      # Example:
      # main = {
      #   font = "JetBrainsMono Nerd Font:size=12";
      #   terminal = "alacritty";
      # };
    };
  };
}
```

**Step 3: Note for manual extraction**

Note: If ~/.config/fuzzel/fuzzel.ini exists, convert it to Nix attributes.
If no config exists, this uses fuzzel defaults.

**Step 4: Commit**

Run:
```bash
git add home/modules/desktop/fuzzel.nix
git commit -m "feat(home/desktop): add fuzzel configuration"
```

---

## Task 26: Extract GTK Configuration

**Files:**
- Create: `home/modules/desktop/gtk.nix`
- Read: `~/.config/gtk-3.0/settings.ini`
- Read: `~/.config/gtk-4.0/settings.ini`

**Step 1: Read GTK settings**

Run:
```bash
cat ~/.config/gtk-3.0/settings.ini 2>/dev/null || echo "No GTK-3 settings"
cat ~/.config/gtk-4.0/settings.ini 2>/dev/null || echo "No GTK-4 settings"
```

Expected: Display GTK settings if they exist

**Step 2: Create gtk.nix**

Create file `home/modules/desktop/gtk.nix`:
```nix
{ config, pkgs, ... }:

{
  gtk = {
    enable = true;

    # Extract theme from gtk settings.ini
    # theme = {
    #   name = "Adwaita-dark";
    #   package = pkgs.gnome-themes-extra;
    # };

    # Extract icon theme
    # iconTheme = {
    #   name = "Adwaita";
    #   package = pkgs.adwaita-icon-theme;
    # };

    # Cursor theme
    cursorTheme = {
      name = "phinger-cursors";
      package = pkgs.phinger-cursors;
    };
  };
}
```

**Step 3: Note for manual extraction**

Note: Extract gtk-theme-name and gtk-icon-theme-name from settings.ini files.
Add appropriate theme packages.

**Step 4: Commit**

Run:
```bash
git add home/modules/desktop/gtk.nix
git commit -m "feat(home/desktop): add gtk configuration (needs manual extraction)"
```

---

## Task 27: Extract Git Configuration

**Files:**
- Create: `home/modules/development/git.nix`
- Read: `~/.gitconfig`
- Read: `~/.config/git/config`

**Step 1: Read git config**

Run:
```bash
cat ~/.gitconfig 2>/dev/null || echo "No .gitconfig"
cat ~/.config/git/config 2>/dev/null || echo "No git/config"
```

Expected: Display git configuration

**Step 2: Create git.nix**

Create file `home/modules/development/git.nix`:
```nix
{ config, pkgs, ... }:

{
  programs.git = {
    enable = true;

    # Extract from .gitconfig
    userName = "Your Name";  # TODO: Replace with actual name
    userEmail = "your@email.com";  # TODO: Replace with actual email

    # Extract aliases
    aliases = {
      # Example:
      # st = "status";
      # co = "checkout";
      # br = "branch";
    };

    # Extract core settings
    extraConfig = {
      core = {
        # editor = "nvim";
      };
      # Add other config sections here
    };
  };
}
```

**Step 3: Note for manual extraction**

Note: Extract user.name, user.email, and aliases from .gitconfig.
DO NOT commit tokens or credentials if present in .gitconfig.

**Step 4: Commit**

Run:
```bash
git add home/modules/development/git.nix
git commit -m "feat(home/development): add git configuration (needs manual extraction)"
```

---

## Task 28: Copy Editor Configurations

**Files:**
- Create: `home/modules/development/editors.nix`
- Create: `home/dotfiles/helix/` (directory for config copies)
- Create: `home/dotfiles/opencode/` (directory for config copies)

**Step 1: Create dotfiles directories**

Run:
```bash
mkdir -p home/dotfiles/helix
mkdir -p home/dotfiles/opencode
```

**Step 2: Copy helix config**

Run:
```bash
cp -r ~/.config/helix/* home/dotfiles/helix/ 2>/dev/null || echo "No helix config to copy"
```

Expected: Helix config files copied if they exist

**Step 3: Copy opencode config**

Run:
```bash
cp -r ~/.config/opencode/* home/dotfiles/opencode/ 2>/dev/null || echo "No opencode config to copy"
```

Expected: OpenCode config files copied if they exist

**Step 4: Create editors.nix**

Create file `home/modules/development/editors.nix`:
```nix
{ config, pkgs, ... }:

{
  # Copy helix configuration
  home.file.".config/helix" = {
    source = ../dotfiles/helix;
    recursive = true;
  };

  # Copy opencode configuration
  home.file.".config/opencode" = {
    source = ../dotfiles/opencode;
    recursive = true;
  };
}
```

**Step 5: Commit**

Run:
```bash
git add home/modules/development/editors.nix home/dotfiles/
git commit -m "feat(home/development): add editor configurations"
```

---

## Task 29: Extract NPM Configuration

**Files:**
- Create: `home/modules/development/node.nix`
- Read: `~/.npmrc`

**Step 1: Read npmrc**

Run:
```bash
cat ~/.npmrc 2>/dev/null || echo "No .npmrc found"
```

Expected: Display npm configuration if exists

**Step 2: Create node.nix**

Create file `home/modules/development/node.nix`:
```nix
{ config, pkgs, ... }:

{
  # NPM configuration
  # If .npmrc exists and contains no secrets, copy it
  # home.file.".npmrc".text = ''
  #   # Extract non-sensitive settings from ~/.npmrc
  # '';

  # If .npmrc contains auth tokens, do NOT commit them
  # Use environment variables or secrets management instead
}
```

**Step 3: Note for security**

Note: DO NOT commit auth tokens from .npmrc.
Only include non-sensitive settings like registry URLs.

**Step 4: Commit**

Run:
```bash
git add home/modules/development/node.nix
git commit -m "feat(home/development): add node/npm configuration"
```

---

## Task 30: First Build Attempt

**Files:**
- None (testing build)

**Step 1: Attempt to build the configuration**

Run:
```bash
nixos-rebuild build --flake .#nixos
```

Expected: Build will likely fail with missing configuration details, but should show what needs to be fixed

**Step 2: Review error messages**

Expected errors might include:
- Missing settings in niri.nix
- Missing settings in noctalia.nix
- Missing details in other configs

**Step 3: Note errors for next steps**

Run:
```bash
nixos-rebuild build --flake .#nixos 2>&1 | tee build-errors.log
```

Expected: Error log saved to build-errors.log for review

**Step 4: Commit build log**

Run:
```bash
git add build-errors.log
git commit -m "chore: add first build attempt error log"
```

---

## Task 31: Manual Configuration Extraction

**Files:**
- Modify: All home manager modules that need manual extraction

**Step 1: Extract Zsh configuration**

1. Open ~/.zshrc
2. Copy aliases to home/modules/shell/zsh.nix shellAliases
3. Copy init commands to initExtra
4. Save and commit: `git commit -am "feat(home/shell): extract zsh configuration from .zshrc"`

**Step 2: Extract Noctalia settings**

1. Open Noctalia Settings Panel → General → "Copy Settings"
2. Paste JSON into home/modules/shell/noctalia.nix settings
3. Save and commit: `git commit -am "feat(home/shell): extract noctalia settings"`

**Step 3: Convert Niri config**

1. Read ~/.config/niri/config.kdl
2. Convert to Nix attributes following niri-flake docs
3. Update home/modules/desktop/niri.nix
4. Save and commit: `git commit -am "feat(home/desktop): convert niri config to nix"`

**Step 4: Convert Alacritty config**

1. Read ~/.config/alacritty/alacritty.toml
2. Convert TOML to Nix attributes
3. Update home/modules/desktop/alacritty.nix
4. Save and commit: `git commit -am "feat(home/desktop): convert alacritty config"`

**Step 5: Extract Git config**

1. Read ~/.gitconfig
2. Extract user.name, user.email, aliases
3. Update home/modules/development/git.nix
4. Save and commit: `git commit -am "feat(home/development): extract git configuration"`

---

## Task 32: Build and Fix Errors

**Files:**
- Various (fixing errors)

**Step 1: Build again**

Run:
```bash
nixos-rebuild build --flake .#nixos
```

Expected: May still have errors, but fewer than before

**Step 2: Fix errors one by one**

For each error:
1. Read the error message
2. Identify the problematic file
3. Fix the issue
4. Commit the fix: `git commit -am "fix: <description>"`
5. Rebuild

**Step 3: Iterate until successful build**

Run:
```bash
nixos-rebuild build --flake .#nixos
```

Expected: Eventually succeeds with "result" symlink created

**Step 4: Verify result**

Run:
```bash
ls -la result
```

Expected: "result" symlink points to /nix/store/... with the system closure

---

## Task 33: Switch to New Configuration

**Files:**
- None (system switch)

**Step 1: Create backup commit**

Run:
```bash
git commit -am "chore: configuration ready for switch"
git tag v0.1.0-pre-switch
```

Expected: Current state saved before switching

**Step 2: Switch to new configuration**

Run:
```bash
sudo nixos-rebuild switch --flake .#nixos
```

Expected: System rebuilds and switches to new configuration
May take several minutes for first build

**Step 3: Verify system still works**

After rebuild:
1. System should not crash
2. You should still be logged in
3. Terminal should work

**Step 4: Reboot to test clean boot**

Run:
```bash
sudo reboot
```

Expected: System reboots successfully

---

## Task 34: Post-Switch Validation

**Files:**
- Create: `VALIDATION.md` (validation checklist)

**Step 1: Create validation document**

Create file `VALIDATION.md`:
```markdown
# Configuration Validation Checklist

Test each item after switching to new configuration:

## System Level
- [ ] System boots successfully
- [ ] Network connectivity works (ping 8.8.8.8)
- [ ] User login works

## Desktop Environment
- [ ] Niri compositor starts
- [ ] Noctalia shell loads
- [ ] Noctalia lockscreen works (test lock/unlock)
- [ ] Alacritty terminal opens
- [ ] Fuzzel launcher works (test opening)
- [ ] Audio works (test playing sound)

## Development Tools
- [ ] Git config correct (git config --list)
- [ ] Docker accessible (docker ps)
- [ ] Node.js works (node --version)
- [ ] Neovim opens
- [ ] Helix config preserved
- [ ] OpenCode config preserved

## Applications
- [ ] Brave browser opens
- [ ] Discord opens
- [ ] All keybinds functional

## Issues Found
(Document any issues here)
```

**Step 2: Test each item**

Go through the checklist systematically

**Step 3: Document issues**

If issues found, add them to VALIDATION.md

**Step 4: Commit validation results**

Run:
```bash
git add VALIDATION.md
git commit -m "chore: add post-switch validation results"
```

---

## Task 35: Fix Post-Switch Issues (If Any)

**Files:**
- Various (depending on issues)

**Step 1: For each issue found in validation**

1. Identify the problematic module
2. Fix the configuration
3. Commit: `git commit -am "fix: <issue description>"`
4. Rebuild: `sudo nixos-rebuild switch --flake .#nixos`
5. Re-test

**Step 2: Update VALIDATION.md**

Mark fixed items as complete

**Step 3: Final commit when all issues resolved**

Run:
```bash
git commit -am "chore: all validation issues resolved"
git tag v1.0.0
```

Expected: Configuration fully working and tagged

---

## Task 36: Add README

**Files:**
- Create: `README.md`

**Step 1: Create README**

Create file `README.md`:
```markdown
# NixOS Configuration - milos_niri

Modular, reproducible NixOS configuration with Home Manager for multiple machines.

## Features

- **Compositor:** Niri with native configuration
- **Shell:** Noctalia with integrated lockscreen
- **Terminal:** Alacritty
- **Desktop:** Wayland + Pipewire
- **Development:** Neovim, Helix, Docker, Node.js 24

## Structure

```
.
├── flake.nix              # Main flake configuration
├── hosts/                 # Host-specific configurations
│   └── nixos/            # Current machine
├── modules/              # Modular system configuration
│   ├── system/           # Boot, networking, locale, users, security
│   ├── desktop/          # Niri, audio, wayland, fonts
│   ├── development/      # Editors, tools, docker
│   └── applications/     # Browsers, terminals, utilities
└── home/                 # Home Manager configuration
    ├── milgraph.nix      # User configuration
    └── modules/          # User dotfile modules
        ├── shell/        # Zsh, Noctalia
        ├── desktop/      # Niri, Alacritty, Fuzzel, GTK
        └── development/  # Git, editors, Node

## Usage

### Build without applying
```bash
nixos-rebuild build --flake .#nixos
```

### Apply configuration
```bash
sudo nixos-rebuild switch --flake .#nixos
```

### Rollback if needed
```bash
sudo nixos-rebuild switch --rollback
```

Or reboot and select previous generation from bootloader.

## Adding a New Machine

1. Boot new machine with NixOS installer
2. Clone this repository
3. Generate hardware config: `nixos-generate-config --root /mnt`
4. Copy `hardware-configuration.nix` to `hosts/<new-machine>/`
5. Create `hosts/<new-machine>/configuration.nix`
6. Update `flake.nix` to add new host
7. Install: `nixos-install --flake .#<new-machine>`

## Notes

- Configuration uses nixpkgs unstable
- Home Manager integrated as NixOS module
- Niri config uses sodiboo/niri-flake for validation
- Noctalia provides built-in lockscreen (no swaylock needed)

## References

- [NixOS Manual](https://nixos.org/manual/nixos/stable/)
- [Home Manager Manual](https://nix-community.github.io/home-manager/)
- [Noctalia Docs](https://docs.noctalia.dev/)
- [Niri Flake](https://github.com/sodiboo/niri-flake)
```

**Step 2: Commit README**

Run:
```bash
git add README.md
git commit -m "docs: add comprehensive README"
```

---

## Task 37: Clean Up and Final Commit

**Files:**
- Remove: `.gitkeep` files
- Remove: `build-errors.log` (if exists)

**Step 1: Remove .gitkeep files**

Run:
```bash
find . -name ".gitkeep" -delete
```

Expected: All .gitkeep files removed (directories now have actual files)

**Step 2: Remove temporary files**

Run:
```bash
rm -f build-errors.log
```

**Step 3: Final commit**

Run:
```bash
git add -A
git commit -m "chore: clean up temporary files"
```

**Step 4: Create final tag**

Run:
```bash
git tag -a v1.0.0 -m "Initial release of reproducible NixOS configuration"
```

Expected: Configuration complete and tagged

---

## Post-Implementation Notes

### Optional Next Steps

1. **Push to remote repository**
   ```bash
   git remote add origin <your-repo-url>
   git push -u origin main
   git push --tags
   ```

2. **Set up secrets management** (if needed)
   - Consider sops-nix or agenix for managing secrets
   - Document in a separate SECRETS.md

3. **Prepare for second PC deployment**
   - Document hardware differences
   - Plan host-specific module overrides

### Maintenance

- Regularly update flake inputs: `nix flake update`
- Rebuild after updates: `sudo nixos-rebuild switch --flake .#nixos`
- Test new configurations in VM before switching
- Keep old generations for rollback capability

### Known Limitations

- Manual extraction required for some configs (zsh, noctalia, niri, alacritty, git)
- Hardware-specific settings in boot.nix may not work on other machines
- Monitor names in niri config are hardware-specific
- Unstable channel means occasional breaking changes
