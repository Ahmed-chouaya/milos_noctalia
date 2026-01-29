{ config, pkgs, ... }:

{
  # Utility applications
  environment.systemPackages = with pkgs; [
    # Launchers
    rofi
    fuzzel

    # Background and notifications
    swaybg
    mako
    swayidle

    # Communication
    discord
    thunderbird
    zoom-us

    # Screen recording
    gpu-screen-recorder
    gpu-screen-recorder-gtk

    # Development tools
    claude-code
    opencode
    vscode

    # Miscellaneous
    phinger-cursors
  ];
}
