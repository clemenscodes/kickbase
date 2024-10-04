{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    nix-filter = {
      url = "github:numtide/nix-filter";
    };
    process-compose-flake = {
      url = "github:Platonic-Systems/process-compose-flake";
    };
    services-flake = {
      url = "github:juspay/services-flake";
    };
  };

  outputs = inputs:
    with inputs;
      flake-parts.lib.mkFlake {inherit inputs;} {
        imports = [
          process-compose-flake.flakeModule
        ];
        systems = [
          "x86_64-linux"
          "aarch64-linux"
        ];
        perSystem = {
          pkgs,
          system,
          ...
        }: let
          assets = pkgs.stdenv.mkDerivation {
            inherit version;
            src = nix-filter.lib {
              root = ./crates/server/.;
              include = [
                "assets"
                "styles"
                "templates"
                "tailwind.config.js"
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
            sha256 = "sha256-VZZnlyP69+Y3crrLHQyJirqlHrTtGTsyiSnZB8jEvVo=";
          };

          pkgs = import nixpkgs {
            inherit system;
            overlays = [(import rust-overlay)];
          };

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          src = nix-filter.lib {
            root = ./.;
            include = [
              ./Cargo.toml
              ./Cargo.lock
              ./taplo.toml
              ./rustfmt.toml
              ./rust-toolchain.toml
              ./deny.toml
              ./crates
            ];
          };

          inherit (craneLib.crateNameFromCargoToml {inherit src;}) pname version;

          args = {
            inherit src;
            strictDeps = true;
            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
          };

          individualCrateArgs =
            args
            // {
              inherit cargoArtifacts version;
              doCheck = false;
            };

          fileSetForCrate = crateFiles:
            nix-filter.lib {
              root = ./.;
              include =
                [
                  ./Cargo.toml
                  ./Cargo.lock
                ]
                ++ crateFiles;
            };

          cargoArtifacts = craneLib.buildDepsOnly args;

          api = craneLib.buildPackage (individualCrateArgs
            // rec {
              pname = "api";
              cargoExtraArgs = "-p ${pname}";
              src = fileSetForCrate [
                ./crates/api/src
                ./crates/api/Cargo.toml
              ];
            });

          server = craneLib.buildPackage (individualCrateArgs
            // rec {
              pname = "server";
              cargoExtraArgs = "-p ${pname}";
              src = fileSetForCrate [
                ./crates/api
                ./crates/server/src
                ./crates/server/templates
                ./crates/server/styles
                ./crates/server/Cargo.toml
              ];
            });

          app = pkgs.writeShellScriptBin pname ''
            WEBSERVER_ASSETS=${assets}/assets ${server}/bin/server
          '';
        in {
          checks = {
            inherit app api server assets;
            inherit (self.packages.${system}) kickbase;

            clippy = craneLib.cargoClippy (args
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              });

            doc = craneLib.cargoDoc (args
              // {
                inherit cargoArtifacts;
              });

            fmt = craneLib.cargoFmt {
              inherit src;
            };

            toml-fmt = craneLib.taploFmt {
              src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
              taploExtraArgs = "--config ./taplo.toml";
            };

            audit = craneLib.cargoAudit {
              inherit src advisory-db;
            };

            deny = craneLib.cargoDeny {
              inherit src;
            };

            nextest = craneLib.cargoNextest (args
              // {
                inherit cargoArtifacts;
                partitions = 1;
                partitionType = "count";
              });

            coverage = craneLib.cargoLlvmCov (args
              // {
                inherit cargoArtifacts;
              });
          };

          packages = {
            inherit app api server assets;
            inherit (self.checks.${system}) coverage;
            default = self.packages.${system}.kickbase;
          };

          apps = {
            default = {
              program = self.packages.${system}.default;
            };
          };

          devShells = {
            default = craneLib.devShell {
              checks = self.checks.${system};
              packages = with pkgs; [
                rust-analyzer
                proto
                moon
                alejandra
                hadolint
                tailwindcss
                bun
                cargo-watch
                cargo-nextest
                cargo-hakari
                taplo
              ];
              RUST_SRC_PATH = "${craneLib.rustc}/lib/rustlib/src/rust/library";
              RUST_BACKTRACE = 1;
            };
          };

          process-compose = {
            kickbase = {
              imports = [
                services-flake.processComposeModules.default
              ];
              settings = {
                processes = {
                  server = {
                    command = "${self.packages.${system}.app}/bin/${pname}";
                  };
                };
              };
            };
          };

          formatter = pkgs.alejandra;
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
