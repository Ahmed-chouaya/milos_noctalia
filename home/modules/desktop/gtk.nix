{ config, pkgs, ... }:

{
  gtk = {
    enable = true;

    # GTK theme
    theme = {
      name = "Adwaita";
      package = pkgs.gnome-themes-extra;
    };

    # Icon theme
    iconTheme = {
      name = "Adwaita";
      package = pkgs.adwaita-icon-theme;
    };

    # Cursor theme
    cursorTheme = {
      name = "phinger-cursors";
      package = pkgs.phinger-cursors;
    };
  };

}
