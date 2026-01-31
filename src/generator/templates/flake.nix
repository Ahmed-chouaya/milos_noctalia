{
  description = "{{ hostname }} - NixOS configuration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/" + "{{ nixpkgs_ref }}";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, home-manager, ... }:
    let
      system = "x86_64-linux";
      username = "{{ username }}";
    in {
      nixosConfigurations = {
        {{ hostname }} = nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            ./modules/nixos
            ./modules/users.nix
            ./modules/git.nix
            ./modules/locale.nix
            ./modules/noctalia.nix
            ./modules/niri
            ./nix.conf
          ];
        };
      };
    };
}
