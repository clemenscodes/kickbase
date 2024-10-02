{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    nix-filter.url = "github:numtide/nix-filter";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          inherit (nixpkgs) lib;
          inherit (manifest) name version;
          overlays = [(import rust-overlay)];
          pkgs = import nixpkgs {inherit system lib overlays;};
          manifest = (lib.importTOML ./Cargo.toml).package;
          filter = nix-filter.lib;
          pname = name;
          craneLib = (crane.mkLib pkgs).overrideToolchain (
            p: p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
          );
          src = filter {
            root = ./.;
            include = [
              ./src
              ./styles
              ./templates
              ./Cargo.lock
              ./Cargo.toml
            ];
          };
          args = {
            inherit src;
            strictDeps = true;
            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
          };
          cargoArtifacts = craneLib.buildDepsOnly (args // {inherit pname version;});
          clippy = craneLib.cargoClippy (args // {inherit cargoArtifacts;});
          crate = craneLib.buildPackage (args // {inherit cargoArtifacts;});
          coverage = craneLib.cargoTarpaulin (args // {inherit cargoArtifacts;});
          assets = pkgs.stdenv.mkDerivation {
            inherit version;
            src = filter {
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
          app = pkgs.writeShellScriptBin pname ''
            WEBSERVER_ASSETS=${assets}/assets ${crate}/bin/${pname}
          '';
        in
          with pkgs; {
            checks = {
              inherit crate clippy coverage;
            };
            packages = {
              inherit crate assets;
              default = app;
            };
            apps = {
              default = flake-utils.lib.mkApp {
                drv = app;
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
          }
      );
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
