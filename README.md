# MILOS - My NixOS Configuration

<p align="center">
  <video src="./assets/demo.mp4" width="100%" controls poster="./assets/preview.png"></video>
  <br>
  <em>☝️ Watch the demo video above</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/NixOS-Unstable-5277C3?style=for-the-badge&logo=nixos&logoColor=white" />
  <img src="https://img.shields.io/badge/Niri-Compositor-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Noctalia-Desktop%20Shell-purple?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Flakes-Enabled-ff69b4?style=for-the-badge" />
</p>

My personal NixOS configuration using [Niri](https://github.com/YaLTeR/niri) as the compositor and [Noctalia](https://github.com/noctalia-dev/noctalia-shell) as the desktop shell.

This is a reproducible system configuration managed with NixOS Flakes and Home Manager. Install it on any machine and get the exact same setup instantly.

## 📋 Requirements

### Hardware
- **CPU:** x86_64 processor (Intel/AMD)
- **RAM:** 4GB minimum, 8GB recommended
- **Storage:** 20GB free space
- **Graphics:** Any GPU with Wayland support

### Prerequisites
- NixOS installed with Flakes enabled
- Internet connection
- 10-30 minutes for first build

## 🚀 Installation

### Step 1: Clone & Enter

```bash
cd ~
git clone https://github.com/YOUR_USERNAME/MILOS.git
cd MILOS
```

### Step 2: Copy Default Templates

Copy the default template files and customize them:

```bash
# Copy the default user configuration
cp home/default.nix home/yourusername.nix

# Copy the default system user configuration
cp modules/system/users.default.nix modules/system/users.nix

# Copy the default host configuration
cp -r hosts/default hosts/yourhostname

# Copy the default flake
cp flake.default.nix flake.nix
```

### Step 3: Edit the Files

Now edit each file you just copied and change the placeholders:

**1. Edit `home/yourusername.nix` (Line 22-23):**
```nix
username = "yourusername";
homeDirectory = "/home/yourusername";
```

**2. Edit `modules/system/users.nix` (Line 8-10):**
```nix
users.users.yourusername = {
  isNormalUser = true;
  description = "Your Full Name";
```

**3. Edit `hosts/yourhostname/configuration.nix` (Line 16):**
```nix
trusted-users = [ "root" "yourusername" ];
```

**4. Edit `flake.nix`:**
- Change hostname: `default = nixpkgs.lib.nixosSystem` → `yourhostname = nixpkgs.lib.nixosSystem`
- Update imports: `./hosts/default/` → `./hosts/yourhostname/`
- Update username: `home-manager.users.yourusername` → match your actual username
- Update home import: `import ./home/default.nix` → `import ./home/yourusername.nix`

**5. Edit `home/modules/development/git.nix`:**
```nix
userName = "YourGitHubUsername";
userEmail = "your.email@example.com";
```

### Step 4: Generate Hardware Config

```bash
sudo nixos-generate-config --show-hardware-config > /tmp/hardware.nix
cp /tmp/hardware.nix hosts/yourhostname/hardware-configuration.nix
```

⚠️ **Don't commit** `hardware-configuration.nix` - it contains system-specific IDs.

### Step 5: Build

```bash
# Replace 'yourhostname' with what you set in flake.nix:
sudo nixos-rebuild switch --flake .#yourhostname
```

First build takes 10-30 minutes.

### Step 6: Enable Desktop

```bash
systemctl --user enable --now noctalia-shell.service
```

Log out and back in, or reboot.

## 🎉 First Steps

1. **Log in** - The desktop loads automatically
2. **Press `Super+O`** - See all windows in overview
3. **Press `Super+D`** - Open the application launcher
4. **Open Settings** - Click the icon in top-right corner
5. **Customize** - Pick a wallpaper or color scheme

## ⌨️ Essential Shortcuts

| Key | Action |
|-----|--------|
| `Super+T` | Open Terminal |
| `Super+D` | Open App Launcher |
| `Super+Q` | Close Window |
| `Super+O` | Show All Windows |
| `Super+Tab` | Switch Window |
| `Super+F` | Fullscreen |
| `Super+Wheel` | Scroll Through Windows |
| `Super+1-9` | Switch Workspace |

*Super = Windows/Command key*

## 🎨 Customization

### Change Appearance

1. Open Settings (top-right icon)
2. Go to "Color Scheme"
3. Choose a wallpaper or preset
4. Apps will update to match

### Add Your Own Apps

**System-wide:**
Edit `modules/applications/utilities.nix`:
```nix
environment.systemPackages = with pkgs; [
  your-app-here
];
```

**Just for your user:**
Edit `home/yourusername.nix`:
```nix
home.packages = with pkgs; [
  your-app-here
];
```

Then rebuild:
```bash
sudo nixos-rebuild switch --flake .#yourhostname
```

### Change Keybindings

Edit `home/dotfiles/niri/config.kdl` and modify the `binds` section.

## 🔧 Troubleshooting

### Build Failed?

```bash
sudo nixos-rebuild switch --flake .#yourhostname --show-trace
```

### Desktop Not Starting?

```bash
systemctl --user status noctalia-shell
systemctl --user restart noctalia-shell
```

### Rollback?

```bash
sudo nixos-rebuild switch --rollback
```

## 🗂️ Structure

```
MILOS/
├── flake.default.nix      # Template - copy to flake.nix and edit
├── flake.nix              # Your actual flake (created from template)
├── hosts/
│   ├── default/           # Template host config
│   │   └── configuration.nix
│   └── yourhostname/      # Your host config (copy from default)
│       ├── configuration.nix  # EDIT THIS
│       └── hardware-configuration.nix  # ⚠️ Don't commit
├── modules/
│   ├── system/
│   │   ├── users.default.nix  # Template - copy to users.nix
│   │   └── users.nix      # Your users config (created from template)
│   ├── desktop/           # Graphics, audio, fonts
│   └── applications/      # Apps for everyone
└── home/
    ├── default.nix        # Template user config
    ├── yourusername.nix   # Your user config (copy from default)
    ├── dotfiles/          # App configs
    └── modules/
        └── development/
            └── git.nix    # EDIT THIS (your git info)
```

**Template files to copy and edit:**
1. ✅ `flake.default.nix` → `flake.nix` (hostname, username, imports)
2. ✅ `modules/system/users.default.nix` → `modules/system/users.nix` (username, description)
3. ✅ `home/default.nix` → `home/yourusername.nix` (username, homeDirectory)
4. ✅ `hosts/default/` → `hosts/yourhostname/` (configuration.nix with username)
5. ✅ `home/modules/development/git.nix` (git name and email)

## 📝 Daily Commands

```bash
# Update everything
nix flake update && sudo nixos-rebuild switch --flake .#yourhostname

# Rebuild after changes
sudo nixos-rebuild switch --flake .#yourhostname

# Clean up old versions
sudo nix-collect-garbage --delete-older-than 7d
```

## 💡 About This Configuration

This is my personal NixOS setup that I can install on any machine and get the same environment instantly. It uses:

- **NixOS** with Flakes for reproducible system builds
- **Niri** as the Wayland compositor for scrollable tiling
- **Noctalia** as the desktop shell providing the bar, widgets, and theming
- **Home Manager** for declarative user configuration

The configuration is fully declarative - everything is defined in these files. Change something, rebuild, and the system updates exactly as specified.

## 📜 License

Use it however you like. This is just my personal configuration shared publicly.

## 🫡 Credits

- [Niri](https://github.com/YaLTeR/niri) - Scrollable-tiling Wayland compositor
- [Noctalia Shell](https://github.com/noctalia-dev/noctalia-shell) - Desktop shell
- [NixOS](https://nixos.org/) - The Linux distribution
- [Home Manager](https://github.com/nix-community/home-manager) - User environment manager

---

<p align="center">
  A reproducible NixOS system. Install anywhere, get the same setup.
</p>
