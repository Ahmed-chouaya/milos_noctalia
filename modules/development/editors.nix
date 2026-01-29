{ config, pkgs, ... }:

{
  # Editor packages
  environment.systemPackages = with pkgs; [
    neovim
  ];
}
