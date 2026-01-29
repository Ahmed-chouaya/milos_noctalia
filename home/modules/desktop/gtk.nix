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

  # Copy custom GTK CSS for color theming
  home.file.".config/gtk-3.0/gtk.css".source = ../dotfiles/gtk/gtk-3.0.css;
  home.file.".config/gtk-4.0/gtk.css".source = ../dotfiles/gtk/gtk-4.0.css;
}
