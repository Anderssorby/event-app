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
          rust =
            with fenix.packages.${system};
            combine [
              complete.cargo
              complete.rustc
              targets.${wasm-target}.latest.rust-std
            ];
          nativeBuildInputs = [
            cargo-binstall
            pkg-config
            #dx needs 0.7.0-rc.2
            rust
          ];
          buildInputs = [ pkgs.openssl ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
        in
        {
          packages.dx = dx;
          devShells.default = mkShell {
            inherit LD_LIBRARY_PATH nativeBuildInputs buildInputs;
            shellHook = ''
              export PATH=~/.cargo/bin:$PATH
              echo "Welcome to the development shell for the Event app!"
            '';
          };
          devShells.android = mkShell {
            inherit LD_LIBRARY_PATH  buildInputs;
            nativeBuildInputs = nativeBuildInputs ++ [
              android-tools
            ];
            shellHook = ''
              export PATH=~/.cargo/bin:$PATH
              echo "Welcome to the android development shell for the Event app!"
            '';
          };
          devShells.ios = mkShell {
            inherit LD_LIBRARY_PATH  buildInputs;
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
