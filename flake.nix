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
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "liushu";
            version = "0.1.0";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
          liushu-db-build = pkgs.rustPlatform.buildRustPackage {
            pname = "liushu-db-build";
            version = "0.1.0";
            src = ./.;
            cargoBuildFlags = [ "-p liushu-db-build" ];
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };

        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            dhall
            (rust-bin.stable."1.71.1".default.override {
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
