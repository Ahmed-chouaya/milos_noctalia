{ config, pkgs, ... }:

{
  # Enable zsh system-wide
  programs.zsh.enable = true;

  # Define user account
  # CHANGE THIS: Update username and description
  users.users.yourusername = {
    isNormalUser = true;
    description = "Your Full Name";
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
