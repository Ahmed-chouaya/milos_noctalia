# Feature Landscape: NixOS TUI Installer

**Domain:** Terminal User Interface installer for NixOS
**Researched:** January 31, 2026
**Confidence:** MEDIUM-HIGH

## Executive Summary

NixOS TUI installers sit at the intersection of two worlds: the manual, complex NixOS installation process and the user-friendly installer experience that modern OSes provide. The ecosystem shows clear patterns in what users expect (table stakes) versus what differentiates products. The key insight is that NixOS installation is fundamentally different from other Linux distributions due to its declarative nature—users must ultimately produce a `configuration.nix` file. TUI installers serve as a guided interface to this process rather than replacing it entirely.

Current market offerings (nixos-wizard, Aegis TUI, Lassulus nixos-installer) share common features around user accounts, disk partitioning, and package selection, but differ significantly in their approach to configuration generation and flake integration. The research reveals that the most successful installers leverage existing tools like `disko` for declarative disk setup and integrate with the broader NixOS ecosystem rather than reinventing core functionality.

---

## Table Stakes

Features users expect. Missing = product feels incomplete or unusable.

### Core System Configuration

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Hostname configuration** | System identity is fundamental; without it, system is unreachable on networks | Low | Must set `networking.hostName` in configuration |
| **User account creation** | Users need non-root access; `root` login disabled by many display managers | Medium | Must create user with `isNormalUser = true`, configure shell, home directory |
| **Password setup** | Required for initial login security; installer must support both root and user passwords | Low | Support interactive input, optionally skip for unattended installs |
| **Locale/language selection** | User environment depends on locale for dates, numbers, formatting | Low | Maps to `i18n.defaultLocale` |
| **Timezone configuration** | Clock must be correct for certificates, logs, and user sanity | Low | Maps to `time.timeZone` |
| **Keyboard layout** | Required for usability; different layouts dramatically affect typing | Low | Maps to `services.xserver.xkb.layout` |

### Disk Operations

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Disk selection** | Users must specify which disk to install to; wrong disk = data loss | Medium | Must list available disks (`lsblk`), prevent dangerous selections |
| **Partition scheme selection** | UEFI vs BIOS require different layouts; users don't know the difference | Medium | Preset options: "Erase disk", "Custom partitioning" |
| **Bootloader configuration** | System won't boot without bootloader; bootloader choice affects dual-boot | High | systemd-boot (UEFI) or GRUB (BIOS/UEFI); requires correct device targeting |
| **Filesystem formatting** | Partitions must be formatted; users don't manually run `mkfs` | Medium | Support ext4, optionally btrfs/ZFS; use labels for `/etc/fstab` independence |
| **Swap configuration** | Required for hibernation and memory pressure; size depends on RAM | Low | Partition or file; map to `swapDevices` |
| **LUKS encryption support** | Data security is expected on laptops and sensitive systems | High | Requires key setup, initrd configuration |

### NixOS-Specific

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Hardware detection/generation** | `nixos-generate-config` produces essential `hardware-configuration.nix` | Medium | Run during install or use `nixos-facter` for better detection |
| **Configuration file generation** | NixOS is declarative; installer must produce valid `configuration.nix` | High | Must generate syntactically correct Nix; handle imports correctly |
| **NixOS installation execution** | Running `nixos-install` actually installs the system | Low | Wrapper around the `nixos-install` command |
| **Flake support** | Modern NixOS uses flakes; installer should support flake-based configs | High | Generate `flake.nix`, handle `nixosConfigurations`, manage `flake.lock` |

### Network

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Network connectivity verification** | Installation downloads ~1GB; network must work | Low | Check `ip a`, warn if down |
| **WiFi configuration** | Laptops often use WiFi; installer must support it | Medium | Integrate with NetworkManager/`wpa_supplicant` |

### User Experience

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Progress feedback** | 15+ minute install requires progress indication | Low | Show current step, download progress, percentage |
| **Error handling with recovery** | Install fails; user must understand what happened | Medium | Catch errors, suggest fixes, allow config editing |
| **Summary before install** | Destructive operation requires confirmation | Low | Show all choices, ask "Are you sure?" |
| **Reboot handling** | Install completes; user must know next steps | Low | Clear instructions to remove USB, reboot |

---

## Differentiators

Features that set products apart. Not expected, but valued when present.

### Git Integration (Your Project's Focus)

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Git credentials configuration** | Developers need Git working immediately for dotfiles, nix configs | Medium | Set `programs.git.userName`, `programs.git.userEmail` in Home Manager or system config |
| **Git SSH key setup** | GitHub/GitLab workflows require SSH keys | Medium | Prompt for key path, configure `~/.ssh/config`, set correct permissions |
| **Home Manager integration** | User configs (dotfiles, git, shells) belong in Home Manager | High | Generate Home Manager module, integrate with flake, support `homeConfigurations` |
| **Dotfiles repository cloning** | Users want their configs immediately | High | Clone user repo post-install, set up symlinks |
| **Pre-configured shell environment** | Devs want zsh/bash with plugins immediately | Medium | Install starship, zsh plugins, fzf, ripgrep |

### Advanced Disk Options

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Btrfs subvolumes and snapshots** | Snapshots enable rollback; subvolumes optimize layout | High | Complex config; nixos-wizard plans this feature |
| **ZFS support** | ZFS offers data integrity, compression, snapshots | Very High | Require ZFS kernel module, handle pool creation |
| **LVM integration** | Flexible volume management for complex layouts | High | Layer on top of partition/encryption |
| **Multi-disk configurations** | RAID, bcache, or separate `/home`/`/var` | Very High | Complex edge case; consider deprioritizing |

### Desktop Environment

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Desktop environment selection** | Users want GNOME, KDE, XFCE, or none | Medium | Preset modules for each; handle unfree drivers |
| **GPU driver configuration** | Hardware acceleration is expected for desktop use | High | Detect NVIDIA vs AMD vs Intel; configure `hardware.opengl` |
| **Display manager selection** | Login screen choice matters (GDM, SDDM, LightDM) | Low | Simple enable flag per DM |
| **Unfree software toggle** | NVIDIA, AMDGPU Pro require unfree; users expect the option | Low | Boolean option to set `allowUnfree = true` |

### Developer Experience

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Shell configuration** | Devs want their shell (zsh/fish) with config | Medium | Generate `.zshrc`/`.config/fish/config.fish` |
| **Terminal emulator selection** | Users have strong terminal preferences | Low | Install selected terminal, set as default |
| **Common dev tools pre-installed** | editors, fzf, ripgrep, bat, eza | Low | Pre-selectable package list |
| **Container runtime setup** | Docker/Podman expected for dev workflows | Medium | Enable service, add user to group |
| **SSH server enablement** | Remote access expected for servers/workstations | Low | Enable `services.sshd` |

### Installation Flexibility

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Remote installation (SSH)** | Install headless servers without console | Medium | nixos-anywhere pattern; requires kexec |
| **VM testing before install** | Validate config in VM before committing | Medium | nixos-anywhere `--vm-test` pattern |
| **Configuration import** | Reuse existing configs instead of starting fresh | High | Parse `configuration.nix`, pre-fill values |
| **Preseed/automatic mode** | Unattended installations for containers/VMs | Medium | Accept config file, run non-interactively |

### Hardware Intelligence

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Automatic hardware detection** | Reduce manual config; detect GPU, CPU, peripherals | Medium | Use `nixos-facter` for comprehensive detection |
| **Laptop-specific configs** | Touchpad, brightness, suspend need config | Medium | Laptop module with common fixes |
| **RAID detection** | Existing RAID arrays should be preserved/used | High | Parse `/proc/mdstat`, offer to configure |

---

## Anti-Features

Features to explicitly NOT build. Common mistakes in this domain.

### Anti-Feature 1: Hardcoded Partition Schemes

**What:** Forcing specific partition layouts without user control.

**Why avoid:** 
- Users have existing multi-boot setups
- Different hardware needs different layouts (UEFI vs BIOS, NVMe vs SSD)
- Server vs desktop use cases differ significantly

**Instead:**
- Offer presets ("Erase disk", "Dual boot", "Custom") 
- Let advanced users specify custom Disko config
- Use Disko for declarative, version-controllable partition configs

### Anti-Feature 2: Reimplementing NixOS Configuration Generation

**What:** Building a custom configuration parser/generator instead of using `nixos-generate-config`.

**Why avoid:**
- `nixos-generate-config` handles hardware detection correctly
- Custom implementation will miss edge cases
- Community won't trust configs from unknown generator
- Maintenance burden of keeping up with nixpkgs changes

**Instead:**
- Use `nixos-generate-config` as the base
- Use `nixos-facter` for better hardware detection
- Generate a minimal `configuration.nix` that imports the generated hardware config
- Overlay user choices on top of generated config

### Anti-Feature 3: Ignoring the Declarative Nature

**What:** Treating the installer as "run once and forget" rather than producing a maintainable configuration.

**Why avoid:**
- NixOS's value proposition is reproducible, version-controlled configs
- Users who care about TUI installers care about configuration management
- Producing a black-box install defeats the purpose of NixOS

**Instead:**
- Always generate a flake with version-controlled configuration
- Show users the generated config before installing
- Suggest committing the config to version control post-install
- Support importing existing configs

### Anti-Feature 4: Missing SSH Access Pattern

**What:** Assuming local console access for all installations.

**Why avoid:**
- Headless servers are common in NixOS deployments
- Cloud providers don't give console access
- Users may prefer to work from their main workstation

**Instead:**
- Support SSH-based installation (nixos-anywhere pattern)
- Generate temporary SSH keys for installation
- Document the SSH-based workflow
- Support both local and remote installation modes

### Anti-Feature 5: Over-abstracting Configuration

**What:** Hiding all NixOS complexity behind a GUI that produces magic configs.

**Why avoid:**
- When things break, users need to understand their config
- Advanced users want to tweak beyond what the UI exposes
- The NixOS ecosystem expects users to understand their config

**Instead:**
- Generate readable, well-commented configuration
- Show users the generated Nix code
- Provide links to relevant documentation
- Offer "Advanced mode" for manual config editing

### Anti-Feature 6: No Flake Support

**What:** Building an installer that only produces traditional `configuration.nix`.

**Why avoid:**
- Flakes are the modern, recommended approach
- Flakes provide reproducibility via `flake.lock`
- Community tools (deploy-rs, colmena, clan) expect flakes
- Future NixOS development assumes flakes

**Instead:**
- Always generate flake-based configuration
- Include `flake.nix` with proper inputs and outputs
- Generate `flake.lock` for reproducible builds
- Support importing flake inputs from existing repositories

---

## Feature Dependencies

```
Installation Flow with Dependencies:

┌─────────────────────────────────────────────────────────────┐
│  Pre-flight                                                │
│  ├── Network connectivity (required for nix install)       │
│  └── Root/sudo access (required for all operations)        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Language/Locale/Timezone                                   │
│  ├── locale → timezone (derived from locale choice)        │
│  └── keyboard → locale (keyboard should match locale)      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Disk Configuration                                         │
│  ├── Disk selection (prerequisite for partitioning)        │
│  ├── Partition scheme → bootloader (BIOS/UEFI detection)   │
│  ├── Encryption → key setup (if LUKS selected)             │
│  └── Filesystem → mount points (formatting order matters)  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  User Configuration                                         │
│  ├── Username → home directory (for file paths)            │
│  ├── Password → SSH key setup (optional)                   │
│  └── Shell → Git config (shell-dependent config paths)     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Hardware Detection                                         │
│  └── nixos-generate-config → hardware-configuration.nix    │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Configuration Generation                                   │
│  ├── imports (hardware config → main config)               │
│  ├── flake.nix → flake.lock (nix flake lock)               │
│  └── user choices → module options                         │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Installation                                               │
│  └── nixos-install → system activation                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Post-install                                               │
│  ├── Git setup (requires user config complete)             │
│  └── Shell config (requires user shell selected)           │
└─────────────────────────────────────────────────────────────┘
```

**Critical Path Dependency:**
1. Network → 2. Disk Selection → 3. User Creation → 4. Hardware Detection → 5. Config Generation → 6. Install

Git credentials and Home Manager are **post-install** features that depend on user configuration being complete.

---

## MVP Recommendation

For a minimum viable product that delivers value while staying focused:

### Phase 1: Core (Table Stakes Only)

**Priority order:**
1. **Language, Locale, Timezone** - Simplest to implement, high value
2. **Hostname** - Essential, no dependencies
3. **User account** - Essential, straightforward
4. **Disk selection + partition preset** - Critical, use Disko
5. **Filesystem + bootloader** - Depends on disk selection
6. **Hardware detection** - Run `nixos-generate-config`
7. **Configuration generation** - Generate flake with all above
8. **Installation** - Run `nixos-install`
9. **Progress + error handling** - Essential for usability
10. **Summary + reboot** - Required UX

### Phase 2: Git Integration (Differentiators)

**Priority order:**
1. **Git user config** - `programs.git` settings in flake
2. **Git SSH key** - Prompt for key, configure SSH directory
3. **Shell configuration** - Basic `.zshrc`/`.bashrc` generation
4. **Home Manager integration** - Generate Home Manager module

### Phase 3: Advanced (If Time Permits)

1. **Desktop environment selection** - Common preset modules
2. **Unfree software toggle** - Simple boolean
3. **WiFi configuration** - Use NetworkManager
4. **Custom Disko config import** - Advanced users

---

## Defer to Post-MVP

| Feature | Reason to Defer |
|---------|----------------|
| **ZFS support** | Very high complexity; edge case for most users |
| **Multi-disk RAID** | Complex, rare use case |
| **Btrfs subvolumes** | Complex configuration; nixos-wizard hasn't shipped it |
| **Remote installation (SSH)** | Different execution model; consider nixos-anywhere integration |
| **VM testing** | Different code path; can integrate later |
| **Configuration import** | Parser complexity; hand-crafted configs are fine for MVP |
| **Laptop-specific tweaks** | Large catalog of edge cases |

---

## Sources

**Official Documentation:**
- [NixOS Manual - Installation](https://nixos.org/manual/nixos/stable/#ch-installation) (HIGH confidence)
- [NixOS Manual - Configuration Options](https://nixos.org/manual/nixos/stable/options) (HIGH confidence)
- [NixOS Wiki - Installation Guide](https://nixos.wiki/wiki/NixOS_Installation_Guide) (MEDIUM confidence)

**Existing Tools:**
- [nixos-wizard (nixos-wizard)](https://github.com/km-clay/nixos-wizard) (MEDIUM confidence)
- [Aegis TUI](https://github.com/Athena-OS/aegis-tui) (MEDIUM confidence)
- [Lassulus nixos-installer](https://github.com/Lassulus/nixos-installer) (LOW confidence - early alpha)

**Ecosystem Tools:**
- [nixos-anywhere](https://github.com/nix-community/nixos-anywhere) (HIGH confidence)
- [disko](https://github.com/nix-community/disko) (HIGH confidence)
- [nixos-facter](https://github.com/nix-community/nixos-facter) (MEDIUM confidence)

**Community Resources:**
- [How I like to install NixOS (declaratively) 2025](https://michael.stapelberg.ch/posts/2025-06-01-nixos-installation-declarative/) (MEDIUM confidence)
- [How I install NixOS in 2025 - Pablo Ovelleiro Corral](https://pablo.tools/blog/random/nixos-install-2025/) (MEDIUM confidence)
- [NixOS Installation Guide - The Linux Cast](https://www.youtube.com/watch?v=PSfc-S2z89o) (LOW confidence)
