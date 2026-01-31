{ config, pkgs, ... }:
{
  programs.git = {
    enable = true;
    userName = "testuser";
    userEmail = "test@example.com";
  };
}
