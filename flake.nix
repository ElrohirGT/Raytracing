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
    ...
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
						genAsset() {
							#    | $6 | 
							# $1 | $2 | $3
							#    | $4 | 
							#    | $5 | 
							magick "$1" -rotate 90 out_left.png
							magick "$3" -rotate -90 out_right.png
							magick -gravity Center out_left.png "$2" out_right.png +smush 0 "$4" "$5" -smush 0 -rotate 180 out1.png
							magick -gravity Center out1.png "$6" -smush 0 -rotate 180 out.png
							rm out_left.png out_right.png out1.png
							mv out.png "../../../Raytracing/imgs/$7"
						}
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
