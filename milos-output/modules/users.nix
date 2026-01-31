{ config, pkgs, ... }:
{
  users.users.testuser = {
    isNormalUser = true;
    description = "Test User";
    extraGroups = [
      "wheel"
      "sudo"
      "audio"
      "video"
      "users"
    ];
  };
}
