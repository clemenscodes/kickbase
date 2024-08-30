{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    lpi = {
      url = "github:cymenix/lpi";
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    lpi,
    ...
  }:
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
          proto
          bun
          nix-output-monitor
          lpi.packages.${pkgs.system}.default
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
              proto setup --no-modify-profile
              proto use
              moon setup
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
