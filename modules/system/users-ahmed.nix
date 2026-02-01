{ config, pkgs, ... }:

{
  # Enable zsh system-wide
  programs.zsh.enable = true;

  # Define user account for ahmed on ElMakina
  users.users.ahmed = {
    isNormalUser = true;
    description = "Ahmed";
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
