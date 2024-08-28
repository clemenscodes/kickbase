{pkgs}:
with pkgs;
  rustPlatform.buildRustPackage {
    pname = "kickbase";
    version = "0.1.0";
    src = ./.;
    cargoHash = "sha256-mDCepyFGz31WFHkTZBLV4gqQZPFRkS8ECyPD8O1X1Dc=";
  }
