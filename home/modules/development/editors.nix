{ config, pkgs, ... }:

{
  # Note: Helix configuration is managed by noctalia-shell's template system
  # to enable dynamic theme switching. Do not add home.file.".config/helix" here
  # as it conflicts with noctalia's ability to write theme files.

  # Copy opencode configuration (to be populated in Task 28)
  home.file.".config/opencode" = {
    source = ../../dotfiles/opencode;
    recursive = true;
  };
}
