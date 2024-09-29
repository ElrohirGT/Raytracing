{
  description = "Raytracing flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nix-formatter-pack = {
      url = "github:Gerschtli/nix-formatter-pack";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    nix-formatter-pack,
  }: let
    forAllSystems = {
      pkgs ? nixpkgs,
      function,
    }:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "x86_64-macos"
        "aarch64-linux"
        "aarch64-darwin"
      ]
      (system:
        function {
          pkgs = import pkgs {
            inherit system;
            config.allowUnfree = true;
            overlays = [
              #inputs.something.overlays.default
            ];
          };
          inherit system;
        });
  in {
    devShells = forAllSystems {
      function = {pkgs, ...}: {
        default = pkgs.mkShell {
          packages = [pkgs.imagemagick];
          shellHook = ''
          '';
        };
      };
    };

    formatter = forAllSystems {
      function = {pkgs, ...}:
        nix-formatter-pack.lib.mkFormatter {
          inherit pkgs;

          config.tools = {
            deadnix.enable = true;
            alejandra.enable = true;
            statix.enable = true;
          };
        };
    };
  };
}
