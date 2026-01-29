{ config, pkgs, ... }:

{
  programs.git = {
    enable = true;

    # TODO: Extract from ~/.gitconfig (Task 31)
    userName = "Your Name";  # PLACEHOLDER
    userEmail = "your@email.com";  # PLACEHOLDER

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
