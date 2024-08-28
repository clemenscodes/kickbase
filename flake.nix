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
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        inherit (nixpkgs) lib;
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system lib overlays;};
        rustToolchain = with pkgs;
          (pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = ["rust-src" "clippy" "llvm-tools"];
          };
        buildInputs = with pkgs; [
          coreutils
          bash
          openssl
          pkg-config
          moon
          biome
          inputs.lpi.packages.${pkgs.system}.default
        ];
        nativeBuildInputs = with pkgs; [
          rustToolchain
          rust-analyzer
        ];
      in {
        packages = {
          default = import ./default.nix {inherit pkgs;};
        };
        apps = {
          default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/kickbase";
          };
        };
        devShells = {
          default = pkgs.mkShell {
            inherit buildInputs nativeBuildInputs;
            RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            shellHook = ''
              moon sync projects
              moon sync hooks
              moon sync codeowners
              export MOON="$(pwd)"
            '';
          };
        };
        formatter = pkgs.alejandra;
      }
    );
}
