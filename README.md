# NixOS Configuration with Niri Compositor

A modular, reproducible NixOS configuration using Flakes and Home Manager, featuring the Niri Wayland compositor.

## Table of Contents

- [Features](#features)
- [Quick Reference](#quick-reference)
- [Installation](#installation)
- [Structure](#structure)
- [Configuration](#configuration)
  - [System Configuration](#system-configuration)
  - [Desktop Environment](#desktop-environment)
  - [User Configuration](#user-configuration)
  - [Dotfiles Structure](#dotfiles-structure)
- [Detailed Configuration](#detailed-configuration)
  - [Network & Security](#network--security)
  - [Shell Configuration](#shell-configuration)
  - [Desktop Applications](#desktop-applications)
  - [Development Environment](#development-environment)
- [Common Customization Tasks](#common-customization-tasks)
- [Updating](#updating)
- [Troubleshooting](#troubleshooting)
- [Maintenance](#maintenance)
- [License](#license)
- [Credits](#credits)

## Features

- **Niri Wayland Compositor** - Modern scrollable-tiling window manager
- **Home Manager Integration** - Declarative user environment management
- **Modular Structure** - Clean separation of system, desktop, development, and application modules
- **PipeWire Audio** - Modern audio server with PulseAudio compatibility
- **Development Environment** - Pre-configured with Git, Node.js, Docker, and editors
- **Secure SSH** - Key-based authentication, root login disabled
- **Firewall Configured** - Development ports and SSH properly secured

## Quick Reference

**Rebuild system after changes:**
```bash
sudo nixos-rebuild switch --flake .#nixos
```

**Update all packages:**
```bash
nix flake update && sudo nixos-rebuild switch --flake .#nixos
```

**Rollback to previous configuration:**
```bash
sudo nixos-rebuild switch --rollback
```

**Key Niri shortcuts:**
- `Super+T` - Terminal
- `Super+D` - App launcher
- `Super+Q` - Close window
- `Super+O` - Overview

## Installation

### Prerequisites

- A NixOS installation with Flakes support
- Git for cloning this repository

### Steps

1. **Clone this repository**
   ```bash
   git clone <your-repo-url>
   cd milos_niri
   ```

2. **Copy your hardware configuration**

   Your hardware configuration is system-specific and contains disk UUIDs unique to your machine. Generate and copy it:

   ```bash
   # Generate hardware configuration for your system
   sudo nixos-generate-config --show-hardware-config > /tmp/hardware-config.nix

   # Copy it to the correct location
   cp /tmp/hardware-config.nix hosts/nixos/hardware-configuration.nix
   ```

   **Important:** Do NOT commit `hardware-configuration.nix` to version control if you plan to make this repository public, as it contains system-specific UUIDs.

3. **Customize personal information**

   Update your Git configuration in `home/modules/development/git.nix`:
   ```nix
   userName = "YourGitHubUsername";
   userEmail = "your.email@example.com";
   ```

   If using a different username, update `modules/system/users.nix`:
   ```nix
   users.users.yourusername = {
     # ...
   };
   ```
   And update the home-manager user path in `flake.nix`:
   ```nix
   home-manager.users.yourusername = import ./home/yourusername.nix;
   ```

4. **Configure SSH keys (if SSH is enabled)**

   Since SSH is configured for key-based authentication only, set up your SSH keys:
   ```bash
   # On your client machine, copy your public key to the server
   ssh-copy-id yourusername@nixos

   # Or manually add your public key to the configuration
   # Edit modules/system/users.nix and add:
   # openssh.authorizedKeys.keys = [ "ssh-ed25519 AAAA..." ];
   ```

5. **Build and activate**
   ```bash
   # Build the configuration
   sudo nixos-rebuild switch --flake .#nixos
   ```

## Structure

```
.
├── flake.nix                    # Main flake configuration
├── flake.lock                   # Dependency locks
├── hosts/
│   └── nixos/
│       ├── configuration.nix    # Host-specific config
│       └── hardware-configuration.nix  # Hardware config (not in git)
├── modules/
│   ├── system/                  # System-level modules
│   │   ├── boot.nix
│   │   ├── locale.nix
│   │   ├── networking.nix
│   │   ├── ssh.nix
│   │   ├── users.nix
│   │   └── security.nix
│   ├── desktop/                 # Desktop environment
│   │   ├── niri.nix
│   │   ├── wayland.nix
│   │   ├── audio.nix
│   │   └── fonts.nix
│   ├── development/             # Development tools
│   │   ├── tools.nix
│   │   ├── editors.nix
│   │   └── docker.nix
│   └── applications/            # User applications
│       ├── browsers.nix
│       ├── terminals.nix
│       └── utilities.nix
└── home/
    ├── milgraph.nix            # Home Manager entry point
    ├── modules/
    │   ├── shell/              # Shell configuration
    │   ├── desktop/            # Desktop user configs
    │   └── development/        # Development configs
    └── dotfiles/               # Dotfile configurations
```

## Configuration

### System Configuration

System-level configuration is in `modules/system/`:
- **boot.nix** - Bootloader and kernel settings
- **networking.nix** - Network and firewall settings (see Network & Security section below)
- **ssh.nix** - SSH daemon configuration with key-based authentication
- **users.nix** - User accounts and groups
- **locale.nix** - Timezone and locale settings
- **security.nix** - Polkit, seatd, and power management

### Desktop Environment

Desktop configuration is in `modules/desktop/`:
- **niri.nix** - Niri compositor settings
- **wayland.nix** - Wayland and Xwayland support
- **audio.nix** - PipeWire audio configuration
- **fonts.nix** - Font packages (JetBrains Mono Nerd Font)

### User Configuration

Home Manager configuration is in `home/`:
- **modules/shell/** - Zsh shell configuration with common aliases
- **modules/desktop/** - User-level desktop configs (alacritty, fuzzel, GTK, niri)
- **modules/development/** - Git, Node.js, editor configurations

### Dotfiles Structure

Application-specific configuration files are stored in `home/dotfiles/`:

```
home/dotfiles/
├── niri/
│   ├── config.kdl          # Niri compositor configuration (keybinds, layout, etc.)
│   └── noctalia.kdl        # Noctalia lock screen integration
├── fuzzel/
│   ├── fuzzel.ini          # Fuzzel application launcher config
│   └── themes/
│       └── noctalia        # Noctalia color theme for fuzzel
├── gtk/
│   ├── gtk-3.0.css         # Custom GTK3 color definitions
│   └── gtk-4.0.css         # Custom GTK4 color definitions
├── helix/
│   └── themes/
│       └── noctalia.toml   # Helix editor color theme
└── opencode/
    └── ...                 # OpenCode configuration
```

## Detailed Configuration

### Network & Security

#### Firewall Configuration (`modules/system/networking.nix`)

The firewall is enabled with the following ports open:

**TCP Ports:**
- Port 22 - SSH access
- Ports 3000-3999 - Common Node.js development servers (React, Vite, etc.)
- Ports 8000-8999 - Python/Django/HTTP test servers

**To add Docker ports:**
```nix
# Edit modules/system/networking.nix
networking.firewall.allowedTCPPorts = [ 22 8080 9000 ];  # Add your ports here
```

**To add UDP ports:**
```nix
networking.firewall.allowedUDPPorts = [ 5353 ];  # mDNS, gaming, VoIP, etc.
```

#### SSH Configuration (`modules/system/ssh.nix`)

SSH is configured with security best practices:
- ✅ Key-based authentication only (no passwords)
- ✅ Root login disabled
- ✅ X11 forwarding disabled

**To enable password authentication (less secure):**
```nix
# Edit modules/system/ssh.nix and uncomment:
services.openssh.settings.PasswordAuthentication = true;
```

**To change SSH port:**
```nix
# Edit modules/system/ssh.nix
services.openssh.ports = [ 2222 ];  # Change from default 22

# Also update networking.nix firewall:
networking.firewall.allowedTCPPorts = [ 2222 ];  # Match the new port
```

### Shell Configuration

#### Zsh (`home/modules/shell/zsh.nix`)

Pre-configured with common aliases:
- `ll` → `ls -alh` (detailed list with human-readable sizes)
- `la` → `ls -A` (list all including hidden files)
- `l` → `ls -CF` (compact list with indicators)
- `..` → `cd ..` (go up one directory)

**To add custom aliases:**
```nix
# Edit home/modules/shell/zsh.nix
shellAliases = {
  ll = "ls -alh";
  gs = "git status";
  # Add your aliases here
};
```

### Desktop Applications

#### Niri Compositor (`home/dotfiles/niri/config.kdl`)

Niri is configured with:
- Phinger cursors theme
- Natural scrolling on touchpad
- Numlock enabled on startup
- Waybar status bar
- Noctalia lock screen integration
- Comprehensive keybindings (Mod = Super/Windows key)

**Key shortcuts:**
- `Mod+T` - Open terminal (Alacritty)
- `Mod+D` - Open application launcher (Fuzzel)
- `Mod+Q` - Close window
- `Mod+O` - Toggle overview
- `Mod+Shift+E` - Quit session

**To modify keybindings:**
Edit `home/dotfiles/niri/config.kdl` and rebuild. The file is extensively commented.

#### Alacritty Terminal (`home/modules/desktop/alacritty.nix`)

Configured with:
- 95% opacity for transparency
- 10px padding
- 11pt font size
- Block cursor with blinking
- 10000 lines scrollback history

**To customize:**
```nix
# Edit home/modules/desktop/alacritty.nix
settings = {
  window.opacity = 1.0;        # Fully opaque
  font.size = 12.0;            # Larger font
  # ... other settings
};
```

#### Fuzzel Launcher (`home/dotfiles/fuzzel/`)

Application launcher with Noctalia color theme integration.

**To customize colors:**
Edit `home/dotfiles/fuzzel/themes/noctalia` and modify the color values.

#### GTK Theme (`home/modules/desktop/gtk.nix`)

Using Adwaita theme and icons with custom color definitions for consistency with Noctalia color scheme.

**To change theme:**
```nix
# Edit home/modules/desktop/gtk.nix
theme = {
  name = "Adwaita-dark";         # Use dark variant
  package = pkgs.gnome-themes-extra;
};
```

### Development Environment

#### Git Configuration (`home/modules/development/git.nix`)

Pre-configured with:
- Username: Ahmed-chouaya
- Email: chouaya.ahmed83@gmail.com

**To add Git aliases:**
```nix
# Edit home/modules/development/git.nix
aliases = {
  st = "status";
  co = "checkout";
  br = "branch";
  ci = "commit";
  # Add your aliases here
};
```

**To set default editor:**
```nix
extraConfig = {
  core = {
    editor = "nvim";  # Or "vim", "code", etc.
  };
};
```

#### Docker

Docker is enabled with the user in the `docker` group for rootless operation.

**Security note:** Being in the docker group grants root-equivalent access. If you don't need Docker, consider removing it from `modules/development/docker.nix`.

### Installed Applications

**Development:**
- Git, Neovim, VSCode, OpenCode, Claude Code
- Node.js 24
- Docker

**Desktop:**
- Brave browser
- Alacritty terminal
- Discord, Thunderbird, Zoom
- GPU Screen Recorder

**Utilities:**
- Rofi, Fuzzel (launchers)
- Swaybg, Mako, Swayidle (Wayland utilities)
- Waybar (status bar)

## Common Customization Tasks

### Adding a New System Package

```nix
# Edit the appropriate module (e.g., modules/applications/utilities.nix)
environment.systemPackages = with pkgs; [
  # existing packages...
  your-new-package
];
```

### Adding a New User Package

```nix
# Edit home/milgraph.nix
home.packages = with pkgs; [
  # existing packages...
  your-new-package
];
```

### Changing Hostname

```nix
# Edit modules/system/networking.nix
networking.hostName = "your-hostname";
```

### Enabling Password Authentication for SSH

```nix
# Edit modules/system/ssh.nix
services.openssh.settings.PasswordAuthentication = true;
```

### Opening Additional Firewall Ports

```nix
# Edit modules/system/networking.nix
networking.firewall.allowedTCPPorts = [ 22 8080 9000 ];  # Add your ports
networking.firewall.allowedUDPPorts = [ 5353 ];          # UDP ports
```

### Modifying Niri Keybindings

Edit `home/dotfiles/niri/config.kdl` and change the bindings in the `binds` section. The file is extensively commented with examples.

## Updating

### Update All Packages

```bash
# Update all flake inputs (nixpkgs, home-manager, niri, noctalia)
nix flake update

# Rebuild and switch
sudo nixos-rebuild switch --flake .#nixos
```

### Update Specific Input

```bash
# Update only nixpkgs
nix flake lock --update-input nixpkgs

# Update only home-manager
nix flake lock --update-input home-manager

# Rebuild after update
sudo nixos-rebuild switch --flake .#nixos
```

### Test Configuration Before Switching

```bash
# Build without switching (safe to test)
sudo nixos-rebuild build --flake .#nixos

# If build succeeds, then switch
sudo nixos-rebuild switch --flake .#nixos
```

### Rollback to Previous Generation

```bash
# List available generations
sudo nix-env --list-generations --profile /nix/var/nix/profiles/system

# Rollback to previous generation
sudo nixos-rebuild switch --rollback

# Or boot into a specific generation from GRUB/systemd-boot menu
```

## Troubleshooting

### Build Errors

Check build logs:
```bash
cat build-errors.log
cat build-result.log
```

View detailed Nix build output:
```bash
sudo nixos-rebuild switch --flake .#nixos --show-trace
```

### Hardware Configuration Issues

Regenerate hardware configuration:
```bash
sudo nixos-generate-config --show-hardware-config > /tmp/hardware-config.nix
cp /tmp/hardware-config.nix hosts/nixos/hardware-configuration.nix
```

### Niri Configuration Issues

Test Niri configuration syntax:
```bash
niri validate ~/.config/niri/config.kdl
```

View Niri logs:
```bash
journalctl --user -u niri -f
```

### SSH Connection Issues

Check SSH service status:
```bash
sudo systemctl status sshd
```

Test SSH from localhost:
```bash
ssh localhost
```

View SSH logs:
```bash
sudo journalctl -u sshd -f
```

### Network/Firewall Issues

Check firewall status:
```bash
sudo nft list ruleset
```

Test if port is open:
```bash
# From another machine
nmap -p 22,3000-3999,8000-8999 <your-ip>
```

### Home Manager Issues

Rebuild only Home Manager:
```bash
home-manager switch --flake .#milgraph
```

Check Home Manager generation:
```bash
home-manager generations
```

### Cleaning Up Old Generations

```bash
# Remove old system generations (keeps last 3)
sudo nix-collect-garbage --delete-older-than 3d

# Optimize nix store
nix-store --optimize
```

## Maintenance

### Best Practices

1. **Test before switching**: Use `sudo nixos-rebuild build --flake .#nixos` to test changes
2. **Commit often**: Track your configuration changes in git
3. **Keep hardware-config separate**: Never commit `hosts/nixos/hardware-configuration.nix`
4. **Update regularly**: Run `nix flake update` monthly to get security patches
5. **Clean old generations**: Periodically run `sudo nix-collect-garbage --delete-older-than 30d`

### Adding New Modules

Create a new module file:
```nix
# modules/system/your-module.nix
{ config, pkgs, ... }:

{
  # Your configuration here
}
```

Add it to `flake.nix`:
```nix
modules = [
  # ... existing modules
  ./modules/system/your-module.nix
];
```

### Git Workflow

```bash
# After making changes
git add .
git commit -m "Description of changes"

# Test the build
sudo nixos-rebuild build --flake .#nixos

# If successful, rebuild
sudo nixos-rebuild switch --flake .#nixos

# Push to remote
git push
```

### Backup Important Files

Before major changes, backup:
- Hardware configuration: `hosts/nixos/hardware-configuration.nix`
- Custom dotfiles in `home/dotfiles/`
- Any local secrets or keys

### Security Checklist

- [ ] Hardware configuration not in version control
- [ ] SSH keys configured (not using password auth)
- [ ] Firewall enabled with only necessary ports
- [ ] Regular system updates applied
- [ ] Docker group membership reviewed (if using Docker)
- [ ] No secrets in Nix configuration files

## License

This configuration is provided as-is for personal use.

## Credits

- [Niri](https://github.com/YaLTeR/niri) - Scrollable-tiling Wayland compositor
- [Home Manager](https://github.com/nix-community/home-manager) - User environment management
- [Noctalia](https://github.com/noctalia/noctalia) - Lock screen and session management
