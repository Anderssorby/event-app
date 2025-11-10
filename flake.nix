{
  description = "Event app";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ fenix, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [

      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        with pkgs;
        let
          dx = pkgs.dioxus-cli;
          wasm-target = "wasm32-unknown-unknown";
          ios-targets = [
            "aarch64-apple-ios"
            "x86_64-apple-ios"
          ];
          ios-toolchains = map (
            target: fenix.packages.${system}.targets.${target}.stable.toolchain
          ) ios-targets;
          android-targets = [
            "aarch64-linux-android"
            "armv7-linux-androideabi"
            "i686-linux-android"
            "x86_64-linux-android"
          ];
          android-toolchains = map (
            target: fenix.packages.${system}.targets.${target}.stable.toolchain
          ) android-targets;
          rust =
            with fenix.packages.${system};
            combine (
              [
                complete.cargo
                complete.rustc
                targets.${wasm-target}.stable.rust-std
              ]
              ++ android-toolchains
            );
          nativeBuildInputs = [
            cargo-binstall
            pkg-config
            rust
            waylandpp.dev
            pkg-config
          ];
          buildInputs = [ pkgs.openssl ];
          androidComposition = androidenv.composeAndroidPackages {
            platformVersions = [
              "34"
              "35"
              "latest"
            ];
            systemImageTypes = [ "google_apis_playstore" ];
            abiVersions = [
              "armeabi-v7a"
              "arm64-v8a"
            ];
            includeNDK = true;
            includeExtras = [ "extras;google;auto" ];
          };
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
          android-emulator = androidenv.emulateApp {
            name = "emulate-MyAndroidApp";
            platformVersion = "28";
            abiVersion = "x86"; # armeabi-v7a, mips, x86_64
            systemImageType = "google_apis_playstore";
          };
          androidsdk = androidenv.androidPkgs.androidsdk;
          ANDROID_HOME = "${androidsdk}/libexec/android-sdk";
          ANDROID_NDK_ROOT = "${ANDROID_HOME}/ndk-bundle";
        in
        {
          packages.dx = dx;
          devShells.default = mkShell {
            inherit LD_LIBRARY_PATH nativeBuildInputs buildInputs;
            NIXPKGS_ACCEPT_ANDROID_SDK_LICENSE = "1";
            shellHook = ''
              export PATH=~/.cargo/bin:$PATH
              echo "Welcome to the development shell for", f the Event app!"
            '';
          };
          devShells.android = mkShell {
            inherit
              LD_LIBRARY_PATH
              buildInputs
              ANDROID_HOME
              ANDROID_NDK_ROOT
              ;
            nativeBuildInputs = nativeBuildInputs ++ [
              android-tools
              androidsdk
              zulu24
            ];
            NIXPKGS_ACCEPT_ANDROID_SDK_LICENSE = "1";
            shellHook = ''
              export PATH=~/.cargo/bin:$PATH
              echo "Welcome to the android development shell for the Event app!"
            '';
          };
          devShells.ios = mkShell {
            inherit LD_LIBRARY_PATH buildInputs;
            nativeBuildInputs = nativeBuildInputs ++ [
              darwin.xcode
            ];
            shellHook = ''
              export PATH=~/.cargo/bin:$PATH
              echo "Welcome to the ios development shell for the Event app!"
            '';
          };
        };
    };
}
