{
  description = "Modular NixOS configuration with Home Manager";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    noctalia = {
      url = "github:noctalia-dev/noctalia-shell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    niri = {
      url = "github:sodiboo/niri-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, home-manager, noctalia, niri, ... }: {
    nixosConfigurations = {
      nixos = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; };
        modules = [
          # Host-specific configuration
          ./hosts/nixos/configuration.nix
          ./hosts/nixos/hardware-configuration.nix

          # System modules
          ./modules/system/boot.nix
          ./modules/system/networking.nix
          ./modules/system/locale.nix
          ./modules/system/users.nix
          ./modules/system/security.nix
          ./modules/system/ssh.nix
          ./modules/system/hardware.nix

          # Desktop environment
          ./modules/desktop/niri.nix
          ./modules/desktop/audio.nix
          ./modules/desktop/wayland.nix
          ./modules/desktop/fonts.nix

          # Development
          ./modules/development/editors.nix
          ./modules/development/tools.nix
          ./modules/development/docker.nix

          # Applications
          ./modules/applications/browsers.nix
          ./modules/applications/terminals.nix
          ./modules/applications/utilities.nix

          # Home Manager as NixOS module
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.extraSpecialArgs = { inherit inputs; };
            home-manager.users.milgraph = import ./home/milgraph.nix;
          }
        ];
      };
    };
  };
}
