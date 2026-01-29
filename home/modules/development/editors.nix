{ config, pkgs, ... }:

{
  # Copy helix configuration (to be populated in Task 28)
  home.file.".config/helix" = {
    source = ../../dotfiles/helix;
    recursive = true;
  };

  # Copy opencode configuration (to be populated in Task 28)
  home.file.".config/opencode" = {
    source = ../../dotfiles/opencode;
    recursive = true;
  };
}
