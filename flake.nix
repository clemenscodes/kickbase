{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = inputs:
    with inputs;
      flake-parts.lib.mkFlake {inherit inputs;} {
        systems = [
          "x86_64-linux"
          "aarch64-linux"
        ];

        perSystem = {
          pkgs,
          system,
          ...
        }: let
          inherit (nixpkgs) lib;
          inherit ((lib.importTOML ./Cargo.toml).package) name version;

          pname = name;

          assets = pkgs.stdenv.mkDerivation {
            inherit version;
            src = nix-filter.lib {
              root = ./.;
              include = [
                ./assets
                ./styles
                ./templates
                ./tailwind.config.js
              ];
            };
            pname = "${pname}-assets";
            buildPhase = ''
              ${pkgs.tailwindcss}/bin/tailwindcss -i styles/tailwind.css -o assets/main.css
            '';
            installPhase = ''
              mkdir -p $out
              mv assets $out/assets
            '';
          };

          rustToolchain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-3jVIIf5XPnUU1CRaTyAiO0XHVbJl12MSx3eucTXCjtE=";
          };

          pkgs = import nixpkgs {
            inherit system;
            overlays = [(import rust-overlay)];
          };

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          args = {
            inherit pname version;
            src = nix-filter.lib {
              root = ./.;
              include = [
                ./src
                ./styles
                ./templates
                ./Cargo.lock
                ./Cargo.toml
              ];
            };
            strictDeps = true;
            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
          };

          mkCrate = platform: args: let
            cargoArtifacts = craneLib.buildDepsOnly args;
            crate = craneLib.buildPackage (args // {inherit cargoArtifacts;});
          in {
            "${platform}-crate" = crate;
            "${platform}-clippy" = craneLib.cargoClippy (args // {inherit cargoArtifacts;});
            "${platform}-coverage" = craneLib.cargoTarpaulin (args // {inherit cargoArtifacts;});
            "${platform}-app" = pkgs.writeShellScriptBin pname ''
              WEBSERVER_ASSETS=${assets}/assets ${crate}/bin/${pname}
            '';
          };

          linux = mkCrate "linux" args;

          windows =
            mkCrate "windows" args
            // {
              doCheck = false;
              CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
              TARGET_CC = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";
              OPENSSL_DIR = "${pkgs.openssl.dev}";
              OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
              OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";
              depsBuildBuild = with pkgs; [
                pkgsCross.mingwW64.stdenv.cc
                pkgsCross.mingwW64.windows.pthreads
              ];
            };
        in
          with pkgs; {
            checks = {
              inherit (linux) linux-crate linux-clippy linux-coverage;
              inherit (windows) windows-crate windows-clippy windows-coverage;
            };
            packages = {
              inherit assets;
              inherit (linux) linux-crate linux-clippy linux-coverage;
              inherit (windows) windows-crate windows-clippy windows-coverage;
              default = linux.linux-app;
            };
            apps = {
              default = flake-utils.lib.mkApp {
                drv = linux.linux-app;
              };
            };
            devShells = {
              default = craneLib.devShell {
                checks = self.checks.${system};
                packages = [
                  rust-analyzer
                  proto
                  moon
                  alejandra
                  hadolint
                  cargo-watch
                  tailwindcss
                  bun
                ];
                RUST_SRC_PATH = "${craneLib.rustc}/lib/rustlib/src/rust/library";
                RUST_BACKTRACE = 1;
              };
            };
            formatter = alejandra;
          };
      };

  nixConfig = {
    extra-substituters = [
      "https://nix-community.cachix.org"
      "https://clemenscodes.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "clemenscodes.cachix.org-1:yEwW1YgttL2xdsyfFDz/vv8zZRhRGMeDQsKKmtV1N18="
    ];
  };
}
