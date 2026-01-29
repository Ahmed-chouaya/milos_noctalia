{ config, pkgs, inputs, ... }:

{
  imports = [
    inputs.noctalia.homeModules.default
  ];

  programs.noctalia-shell = {
    enable = true;

    # Enable systemd service
    systemd.enable = true;

    # Configuration settings (to be populated in Task 31)
    settings = {
      # General settings
      general = {
        lockOnSuspend = true;
        showSessionButtonsOnLockScreen = true;
        showHibernateOnLockScreen = false;
        compactLockScreen = false;
        enableLockScreenCountdown = true;
        lockScreenCountdownDuration = 10000;
      };

      # TODO: Add more settings from ~/.config/noctalia/
      # Use "Copy Settings" feature in Noctalia settings panel
    };
  };
}
