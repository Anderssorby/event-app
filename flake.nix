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
        in
        {
          packages.dx = dx;
          devShells.default = mkShell {
            nativeBuildInputs = [
              cargo-binstall
              dx
              rust
              # wasm-bindgen-cli
            ];
          };
        };
    };
}
