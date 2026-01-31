{ inputs, config, lib, ... }:
{
  # Noctalia shell configuration

  # Wallpaper directory
  home.file.".config/noctalia/wallpapers".source = "{{ wallpaper_dir }}";

  # Noctalia settings
  programs.noctalia = {
    enable = true;
    wallpaperDir = "{{ wallpaper_dir }}";
  };
}
