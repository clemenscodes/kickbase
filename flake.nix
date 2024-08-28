{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    lpi = {
      url = "github:cymenix/lpi";
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        formatter = pkgs.alejandra;
        devShells = {
          default = mkShell {
            buildInputs = [
              moon
              biome
              gum
              nix-output-monitor
              nvd
              inputs.lpi.packages.${pkgs.system}.default
            ];
            shellHook = ''
              moon sync projects
              moon sync hooks
              moon sync codeowners
              export MOON="$(pwd)"
            '';
          };
        };
      }
    );
}
