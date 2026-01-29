{ config, pkgs, ... }:

{
  # Enable zsh system-wide
  programs.zsh.enable = true;

  # Define user account
  users.users.milgraph = {
    isNormalUser = true;
    description = "Ahmed Chouaya";
    extraGroups = [
      "networkmanager"
      "wheel"
      "audio"
      "video"
      "docker"
      "seat"
      "input"
    ];
    shell = pkgs.zsh;
    packages = with pkgs; [];
  };
}
