{ config, pkgs, inputs, lib, ... }:

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
    # CHANGE THIS: Your username
    username = "yourusername";
    homeDirectory = "/home/yourusername";
    stateVersion = "25.11";

    # User-specific packages (empty for now, packages in system)
    packages = with pkgs; [];
  };

  # Let Home Manager manage itself
  programs.home-manager.enable = true;

  # Force overwrite existing config files that were created outside Home Manager
  # This prevents activation failures when files already exist
  xdg.configFile."alacritty/alacritty.toml".force = true;

  # Noctalia config files: Let home-manager generate them from noctalia.nix,
  # then copy to writable location so noctalia can modify them at runtime
  home.activation.noctaliaWritableConfigs = lib.hm.dag.entryAfter ["writeBoundary"] ''
    # Create noctalia config directory if it doesn't exist
    mkdir -p "$HOME/.config/noctalia"

    # Copy settings.json from nix store to writable location
    if [ -L "$HOME/.config/noctalia/settings.json" ] || [ ! -e "$HOME/.config/noctalia/settings.json" ]; then
      # Backup existing if it's a file (not symlink)
      if [ -f "$HOME/.config/noctalia/settings.json" ] && [ ! -L "$HOME/.config/noctalia/settings.json" ]; then
        cp -f "$HOME/.config/noctalia/settings.json" "$HOME/.config/noctalia/settings.json.backup.$(date +%Y%m%d%H%M%S)"
      fi
      # Copy from nix store (follow symlink) to real file
      cp -L -f "$HOME/.config/noctalia/settings.json" "$HOME/.config/noctalia/settings.json.tmp" 2>/dev/null || true
      if [ -f "$HOME/.config/noctalia/settings.json.tmp" ]; then
        mv -f "$HOME/.config/noctalia/settings.json.tmp" "$HOME/.config/noctalia/settings.json"
        chmod +w "$HOME/.config/noctalia/settings.json"
      fi
    fi

    # Copy plugins.json from nix store to writable location
    if [ -L "$HOME/.config/noctalia/plugins.json" ] || [ ! -e "$HOME/.config/noctalia/plugins.json" ]; then
      # Backup existing if it's a file (not symlink)
      if [ -f "$HOME/.config/noctalia/plugins.json" ] && [ ! -L "$HOME/.config/noctalia/plugins.json" ]; then
        cp -f "$HOME/.config/noctalia/plugins.json" "$HOME/.config/noctalia/plugins.json.backup.$(date +%Y%m%d%H%M%S)"
      fi
      # Copy from nix store (follow symlink) to real file
      cp -L -f "$HOME/.config/noctalia/plugins.json" "$HOME/.config/noctalia/plugins.json.tmp" 2>/dev/null || true
      if [ -f "$HOME/.config/noctalia/plugins.json.tmp" ]; then
        mv -f "$HOME/.config/noctalia/plugins.json.tmp" "$HOME/.config/noctalia/plugins.json"
        chmod +w "$HOME/.config/noctalia/plugins.json"
      fi
    fi
  '';
}
