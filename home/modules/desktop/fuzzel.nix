{ config, pkgs, ... }:

{
  programs.fuzzel = {
    enable = true;
  };

  # Copy fuzzel configuration files from dotfiles
  home.file.".config/fuzzel/fuzzel.ini".source = ../dotfiles/fuzzel/fuzzel.ini;
  home.file.".config/fuzzel/themes/noctalia".source = ../dotfiles/fuzzel/themes/noctalia;
}
