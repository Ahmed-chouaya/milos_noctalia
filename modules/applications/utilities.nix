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

    # Screenshots and screen recording
    grim          # Screenshot utility for Wayland
    slurp         # Region selection for Wayland
    swappy        # Screenshot editor
    gpu-screen-recorder
    gpu-screen-recorder-gtk

    # Clipboard
    wl-clipboard  # Already installed via tools.nix, but listing here for clarity
    cliphist      # Clipboard manager with history

    # File management
    xfce.thunar           # Lightweight file manager
    xfce.thunar-volman    # Volume management for thunar
    xfce.thunar-archive-plugin  # Archive support
    file-roller           # Archive manager

    # System monitoring
    htop          # Process viewer
    btop          # Modern resource monitor

    # Development tools
    claude-code
    opencode
    vscode

    # Media
    mpv           # Video player
    imv           # Image viewer

    # Miscellaneous
    phinger-cursors
  ];
}
