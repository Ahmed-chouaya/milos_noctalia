{ config, pkgs, ... }:

{
  # Development tools
  environment.systemPackages = with pkgs; [
    git
    wget
    nodejs_24
    xdg-utils
    wl-clipboard
  ];
}
