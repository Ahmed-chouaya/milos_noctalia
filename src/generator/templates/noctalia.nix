{ inputs, config, lib, ... }:
{
  # Noctalia shell configuration

  # Wallpaper directory
  home.file.".config/noctalia/wallpapers".source = "{{ wallpaper_dir }}";

  # Avatar section (only included if avatar is set)
  {%- if avatar_path_set -%}
  home.file.".config/noctalia/avatar".source = "{{ avatar_path }}";
  {%- endif -%}

  # Noctalia settings
  programs.noctalia = {
    enable = true;
    wallpaperDir = "{{ wallpaper_dir }}";
    {%- if avatar_path_set -%}
    avatar = "{{ avatar_path }}";
    {%- endif -%}
  };
}
