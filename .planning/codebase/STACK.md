# Technology Stack

**Analysis Date:** 2026-01-31

## Languages

**Primary:**
- **Nix Expression Language** ( Nix ) - Configuration DSL for NixOS and Home Manager
- **KDL** ( Configuration Document Language ) - For Niri compositor configuration
- **TOML** - For Alacritty terminal configuration
- **JSON** - For OpenCode configuration files
- **Shell** ( Bash/Zsh ) - For initialization scripts and aliases

**Secondary:**
- **CSS** - For GTK theme customization
- **Nixlang** ( Nix flake format ) - For flake.nix configuration

## Runtime

**Environment:**
- **NixOS** ( Linux ) - Operating system
- **x86_64-linux** - Target architecture

**Package Manager:**
- **Nix** ( with Flakes ) - Primary package manager
- **Home Manager** - User environment package manager
- Lockfile: `flake.lock` ( present, committed to git )

## Frameworks

**Core:**
- **NixOS** - Linux distribution with declarative system configuration
- **Home Manager** - User environment management (integrated as NixOS module)
- **Niri** - Scrollable-tiling Wayland compositor
- **Noctalia** - Lock screen and session management (from noctalia-dev/noctalia-shell)

**Desktop/UI:**
- **Wayland** - Display protocol (with Xwayland for X11 apps)
- **PipeWire** - Audio server (with PulseAudio compatibility)
- **GTK 3/4** - UI toolkit with custom CSS
- **XDG Desktop Portal** - For screen sharing, file pickers

**Development:**
- **Node.js 24** - JavaScript runtime
- **Git** - Version control
- **Docker** - Container platform (rootless)
- **Neovim** - Text editor

## Key Dependencies

**Critical:**
- **nixpkgs/nixos-unstable** - Unstable channel for latest packages
  - Source: `github:nixos/nixpkgs/nixos-unstable`
  - Purpose: All packages and NixOS modules

- **home-manager** - User environment management
  - Source: `github:nix-community/home-manager`
  - Purpose: Declarative user configuration
  - Inputs: Follows nixpkgs

- **niri-flake** - Niri compositor flake
  - Source: `github:sodiboo/niri-flake`
  - Purpose: Niri package with flake support
  - Inputs: Follows nixpkgs

- **noctalia-shell** - Lock screen solution
  - Source: `github:noctalia-dev/noctalia-shell`
  - Purpose: Lock screen, session management, status bar
  - Inputs: Follows nixpkgs

**Infrastructure:**
- **NetworkManager** - Network management
- **systemd** - Init system and service management
- **PipeWire** - Audio/video processing
- **Xwayland** - X11 emulation for Wayland

## Key Applications Installed

**Desktop Environment:**
- Niri (Wayland compositor)
- Waybar (status bar, via Noctalia)
- Noctalia (lock screen/session manager)
- Alacritty (terminal)
- Fuzzel (application launcher)
- Mako (notifications)
- Swaybg (wallpaper)
- Swayidle (idle management)

**Development:**
- Git
- Neovim
- Node.js 24
- Docker
- VSCode
- Claude Code
- OpenCode

**Communication:**
- Discord
- Thunderbird
- Zoom

**Productivity:**
- Thunar (file manager)
- Grim/Slurp (screenshots)
- GPU Screen Recorder

## Configuration

**Environment:**
- Configured via: NixOS modules and Home Manager
- State version: 25.11 (both system and home)
- Unfree packages: Enabled (`nixpkgs.config.allowUnfree = true`)
- Experimental features: `nix-command`, `flakes`

**Build:**
- Build tool: `nix build` / `nixos-rebuild`
- Flake output: `nixosConfigurations.nixos`
- Configuration format: Nix expression files (`.nix`)

## Platform Requirements

**Development:**
- NixOS with flakes support
- Git for cloning repository
- Sudo access for system changes

**Production:**
- x86_64-linux architecture
- Intel CPU (microcode updates configured)
- UEFI boot (EFI system partition)
- Ext4 root filesystem

## Desktop Integration

**Wayland Stack:**
- Niri compositor (wlroots-based)
- XDG Portal (wlr implementation)
- libinput for input devices
- Phinger cursors theme

**Audio:**
- PipeWire with WirePlumber
- PulseAudio compatibility layer
- Volume control via media keys

**Display:**
- 1920x1080@120Hz on eDP-1 (built-in display)
- Scale: 2x
- Top strut for status bar (40px)

## Security Stack

**SSH:**
- Key-based authentication only
- Root login disabled
- X11 forwarding disabled

**Firewall:**
- nftables based
- Ports 22 (SSH), 3000-3999 (Node.js), 8000-8999 (Python) open

**User Groups:**
- networkmanager, wheel, audio, video, docker, seat, input

---

*Stack analysis: 2026-01-31*
