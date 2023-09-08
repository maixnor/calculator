{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.nightly.latest.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs =
            [ 
              (rustVersion.override { extensions = [ "rust-src" "rust-analyzer" "clippy" "cargo" "rust-docs" "llvm-tools"]; })
              pkgs.helix
              pkgs.nil
            ];
        };
      });
}
