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
          config = {
            android_sdk.accept_license = true;
            allowUnfree = true;
          };
        };

        buildToolsVersion = "31.0.0";
        cmakeVersion = "3.18.1";
        ndkVersion = "25.2.9519653";
        androidComposition = pkgs.androidenv.composeAndroidPackages {
          toolsVersion = "26.1.1";
          platformToolsVersion = "33.0.3";
          buildToolsVersions = [ buildToolsVersion ];
          includeEmulator = false;
          platformVersions = [ "33" ];
          includeSources = false;
          includeSystemImages = false;
          systemImageTypes = [ "google_apis_playstore" ];
          abiVersions = [ "armeabi-v7a" "arm64-v8a" ];
          cmakeVersions = [ cmakeVersion ];
          includeNDK = true;
          ndkVersions = [ ndkVersion ];
          useGoogleAPIs = false;
          useGoogleTVAddOns = false;
          includeExtras = [
            "extras;google;gcm"
          ];
        };
        android-sdk = androidComposition.androidsdk;
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

        devShells.default = mkShell rec {
          ANDROID_SDK_ROOT = "${android-sdk}/libexec/android-sdk";
          NDK_VERSION = ndkVersion;
          ANDROID_NDK_ROOT = "${ANDROID_SDK_ROOT}/ndk-bundle";
          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${ANDROID_SDK_ROOT}/build-tools/${buildToolsVersion}/aapt2";
          JAVA_HOME = "${pkgs.jdk17.home}";

          shellHook = ''
            export PATH="$ANDROID_SDK_ROOT/cmake/${cmakeVersion}/bin:$PATH"
            echo sdk.dir=$ANDROID_SDK_ROOT > liushu-android/local.properties
          '';

          buildInputs = [
            android-sdk
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
