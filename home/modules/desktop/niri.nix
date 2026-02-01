{ config, pkgs, inputs, lib, ... }:

{
  imports = [
    inputs.niri.homeModules.config
  ];

  # Copy niri configuration files from dotfiles to nix store
  # The activation script below will copy it to a writable location
  home.file.".config/niri/config.kdl.source" = {
    source = ../../dotfiles/niri/config.kdl;
  };

  # Activation script: Copy the config from nix store to a real file
  # This allows noctalia-shell to modify it for theme changes
  home.activation.niriWritableConfig = lib.hm.dag.entryAfter ["writeBoundary"] ''
    # Copy config from nix store to writable location
    if [ -L "$HOME/.config/niri/config.kdl" ] || [ ! -f "$HOME/.config/niri/config.kdl" ]; then
      cp -f "$HOME/.config/niri/config.kdl.source" "$HOME/.config/niri/config.kdl"
      chmod +w "$HOME/.config/niri/config.kdl"
    fi
  '';
}
