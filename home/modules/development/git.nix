{ config, pkgs, ... }:

{
  programs.git = {
    enable = true;

    userName = "Ahmed-chouaya";
    userEmail = "chouaya.ahmed83@gmail.com";

    aliases = {
      st = "status";
      co = "checkout";
      br = "branch";
      ci = "commit";
      lg = "log --oneline --graph --decorate";
    };

    extraConfig = {
      core = {
        editor = "nvim";
      };
      init = {
        defaultBranch = "main";
      };
      pull = {
        rebase = false;
      };
    };
  };
}
