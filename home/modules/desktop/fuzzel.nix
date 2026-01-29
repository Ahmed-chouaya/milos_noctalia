{ config, pkgs, ... }:

{
  programs.fuzzel = {
    enable = true;

    # Settings (to be populated in Task 31)
    settings = {
      # TODO: Extract from ~/.config/fuzzel/fuzzel.ini if exists
    };
  };
}
