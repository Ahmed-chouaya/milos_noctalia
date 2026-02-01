{ config, pkgs, lib, ... }:

{
  programs.fuzzel = {
    enable = true;
  };

  # Copy fuzzel configuration files from dotfiles to nix store
  # The activation script below will copy them to writable locations
  home.file.".config/fuzzel/fuzzel.ini.source".source = ../../dotfiles/fuzzel/fuzzel.ini;
  home.file.".config/fuzzel/themes/noctalia.source".source = ../../dotfiles/fuzzel/themes/noctalia;

  # Activation script: Copy the configs from nix store to real files
  # This allows noctalia-shell to modify them for theme changes
  home.activation.fuzzelWritableConfigs = lib.hm.dag.entryAfter ["writeBoundary"] ''
    # Create fuzzel config directory if it doesn't exist
    mkdir -p "$HOME/.config/fuzzel/themes"

    # Copy fuzzel.ini from nix store to writable location
    if [ -L "$HOME/.config/fuzzel/fuzzel.ini" ] || [ ! -e "$HOME/.config/fuzzel/fuzzel.ini" ]; then
      cp -L -f "$HOME/.config/fuzzel/fuzzel.ini.source" "$HOME/.config/fuzzel/fuzzel.ini.tmp" 2>/dev/null || true
      if [ -f "$HOME/.config/fuzzel/fuzzel.ini.tmp" ]; then
        mv -f "$HOME/.config/fuzzel/fuzzel.ini.tmp" "$HOME/.config/fuzzel/fuzzel.ini"
        chmod +w "$HOME/.config/fuzzel/fuzzel.ini"
      fi
    fi

    # Copy theme from nix store to writable location
    if [ -L "$HOME/.config/fuzzel/themes/noctalia" ] || [ ! -e "$HOME/.config/fuzzel/themes/noctalia" ]; then
      cp -L -f "$HOME/.config/fuzzel/themes/noctalia.source" "$HOME/.config/fuzzel/themes/noctalia.tmp" 2>/dev/null || true
      if [ -f "$HOME/.config/fuzzel/themes/noctalia.tmp" ]; then
        mv -f "$HOME/.config/fuzzel/themes/noctalia.tmp" "$HOME/.config/fuzzel/themes/noctalia"
        chmod +w "$HOME/.config/fuzzel/themes/noctalia"
      fi
    fi
  '';
}
