{
  description = "NixOS configuration for Test User";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { inputs, ... }: {
    nixosConfigurations = {
      testhost = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          home-manager.nix
          { home-manager.users.testuser = { imports = [ ./home.nix ]; }; }
        ];
      };
    };
  };
}
