# Coding Conventions

**Analysis Date:** 2026-01-31

## Naming Patterns

**Files:**
- Modules: `lowercase_with_underscores.nix` (e.g., `hardware.nix`, `networking.nix`)
- Host configs: `configuration.nix`, `hardware-configuration.nix`
- Dotfiles: Application-specific names (e.g., `config.kdl`, `fuzzel.ini`, `gtk-3.0.css`)

**Functions:**
- Not applicable (Nix uses attribute sets, not functions)

**Variables:**
- Module arguments: `{ config, pkgs, ... }` or `{ config, pkgs, inputs, ... }`
- Package lists: `with pkgs; [ package1 package2 ]`
- Local bindings: `let name = value; in ...`

**Types:**
- Options: `users.users.<name>`, `services.openssh.<setting>`
- Attributes: `environment.systemPackages`, `programs.git.settings`

## Code Style

**Formatting:**
- Indentation: 2 spaces (Nix convention)
- Line breaks: After `{` and `=` for nested attrsets
- No trailing commas (Nix allows but often omitted)

**Example module structure:**

```nix
{ config, pkgs, ... }:

{
  # Comments use # for single-line
  # Top-level option
  option.name = value;
  
  # Nested attribute set
  services.service = {
    enable = true;
    setting = "value";
  };
  
  # Package list
  environment.systemPackages = with pkgs; [
    package1
    package2
  ];
}
```

## Import Organization

**Order in flake.nix:**

1. Host configurations (`./hosts/nixos/*.nix`)
2. System modules (`./modules/system/*.nix`)
3. Desktop modules (`./modules/desktop/*.nix`)
4. Development modules (`./modules/development/*.nix`)
5. Application modules (`./modules/applications/*.nix`)
6. Home Manager module

**Imports in home/milgraph.nix:**

1. Shell modules (`./modules/shell/*.nix`)
2. Desktop modules (`./modules/desktop/*.nix`)
3. Development modules (`./modules/development/*.nix`)

**Pattern:**
```nix
imports = [
  ./path/to/module1.nix
  ./path/to/module2.nix
];
```

## Module Argument Patterns

**System modules:**
```nix
{ config, pkgs, ... }:
```

**Home Manager modules with flake inputs:**
```nix
{ config, pkgs, inputs, ... }:
```

**Why:** Access to `inputs` for importing external flake modules (like Noctalia)

## Error Handling

**Patterns:**
- Use `mkIf` for conditional configuration:

```nix
services.service.enable = mkIf config.enableFeatureX true;
```

- Use `mkDefault` for setting defaults:

```nix
nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";
```

- Use `mkForce` to override:

```nix
xd.portal.niri.default = pkgs.lib.mkForce [ "wlr" "gtk" ];
```

## Comments

**When to Comment:**
- Explain why non-obvious settings are used
- Document TODO items
- Reference external documentation

**Patterns:**
```nix
# Enable X server (for Xwayland support)
services.xserver.enable = true;

# TODO: After setting up SSH keys, change this to false
services.openssh.settings.PermitRootLogin = "yes";
```

## Function Design

**Module size:** One concern per module file

**Parameters:** Standard `{ config, pkgs, ... }` pattern

**Return values:** Nix attribute set with options

**Example:**
```nix
{ config, pkgs, ... }:

{
  environment.systemPackages = with pkgs; [ git wget ];
}
```

## Module Design

**Single responsibility:** Each module handles one area

**Naming convention:** `<category>/<concern>.nix`

**Dependencies:** Implicit via flake.nix module imports

**No circular imports:** Nix prevents this at evaluation time

## Package Installation Patterns

**System packages:**
```nix
environment.systemPackages = with pkgs; [
  package1
  package2
];
```

**User packages (Home Manager):**
```nix
home.packages = with pkgs; [
  package1
  package2
];
```

**Programs (Home Manager):**
```nix
programs.programName = {
  enable = true;
  settings = { ... };
};
```

## Service Configuration Patterns

**Enable service:**
```nix
services.serviceName.enable = true;
```

**Configure service:**
```nix
services.serviceName = {
  enable = true;
  setting = value;
};
```

**With nested settings:**
```nix
services.serviceName.settings = {
  key = "value";
  nested.key = "value";
};
```

## Environment Variables

**System-wide (NixOS module):**
```nix
environment.sessionVariables = {
  NIXOS_OZONE_WL = "1";
  MOZ_ENABLE_WAYLAND = "1";
};
```

**Per-user (Home Manager):**
```nix
programs.zsh.initContent = ''
  export VAR="value"
'';
```

## XDG File Management

**Home Manager pattern:**
```nix
xdg.configFile."app/config".source = ./dotfiles/app/config;
```

**With force overwrite:**
```nix
xdg.configFile."alacritty/alacritty.toml".force = true;
```

## Git Integration

**User config (Home Manager):**
```nix
programs.git = {
  enable = true;
  settings = {
    user.name = "Username";
    user.email = "email@example.com";
    alias.st = "status";
  };
};
```

## Deprecated Option Handling

**Current approach:** Warnings logged, options still work

**Example warnings observed:**
- `programs.zsh.initExtra` → use `initContent`
- `programs.git.aliases` → use `programs.git.settings.alias`
- `programs.git.userEmail` → use `programs.git.settings.user.email`

**Migration pattern:**
```nix
# Old (deprecated)
programs.zsh.initExtra = "content";

# New (current)
programs.zsh.initContent = "content";
```

---

*Convention analysis: 2026-01-31*
