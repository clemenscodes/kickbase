{pkgs}:
with pkgs;
  rustPlatform.buildRustPackage {
    pname = "kickbase";
    version = "0.1.1";
    src = ./.;
    cargoHash = "sha256-TUI3rdNuErqgruLMvCsT2sltPqtaUbicfe9PkGG6fD4=";
    # cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
  }
