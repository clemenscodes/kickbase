{pkgs}:
with pkgs;
  rustPlatform.buildRustPackage {
    pname = "kickbase";
    version = "0.1.1";
    src = ./.;
    cargoHash = "sha256-0Erpt5rqYTZqBsM3xhCsliN6h73W5BoTRubFthGEuz8=";
    # cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
  }
