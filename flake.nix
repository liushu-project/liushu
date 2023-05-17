{
  description = "Reproduceable dev environment";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            dhall
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [
                "armv7-linux-androideabi"
                "i686-linux-android"
                "x86_64-linux-android"
                "aarch64-linux-android"
              ];
            })
          ];
        };
      }
    );
}
