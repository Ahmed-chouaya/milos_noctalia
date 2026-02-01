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
    username = "ahmed";
    homeDirectory = "/home/ahmed";
    stateVersion = "25.11";

    # User-specific packages (empty for now, packages in system)
    packages = with pkgs; [];
  };

  # Let Home Manager manage itself
  programs.home-manager.enable = true;

  # Force overwrite existing config files that were created outside Home Manager
  # This prevents activation failures when files already exist
  xdg.configFile."alacritty/alacritty.toml".force = true;
}
