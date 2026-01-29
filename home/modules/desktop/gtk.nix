{ config, pkgs, ... }:

{
  gtk = {
    enable = true;

    # TODO: Extract theme from ~/.config/gtk-3.0/settings.ini
    # theme = {
    #   name = "Adwaita-dark";
    #   package = pkgs.gnome-themes-extra;
    # };

    # TODO: Extract icon theme
    # iconTheme = {
    #   name = "Adwaita";
    #   package = pkgs.adwaita-icon-theme;
    # };

    # Cursor theme
    cursorTheme = {
      name = "phinger-cursors";
      package = pkgs.phinger-cursors;
    };
  };
}
