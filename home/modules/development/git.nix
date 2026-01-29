{ config, pkgs, ... }:

{
  programs.git = {
    enable = true;

    userName = "Ahmed-chouaya";
    userEmail = "chouaya.ahmed83@gmail.com";

    # TODO: Extract aliases from ~/.gitconfig
    aliases = {
    };

    # TODO: Extract additional config
    extraConfig = {
      core = {
        # editor = "nvim";
      };
    };
  };
}
