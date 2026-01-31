{ config, pkgs, ... }:
{
  imports = [
    (
      { _module.args = { inputs, ... }; }
      inputs.noctalia.homeModule
    )
  ];

  noctalia = {
    enable = true;
    wallpaperDir = "~/Pictures/Wallpapers";
  };
}
