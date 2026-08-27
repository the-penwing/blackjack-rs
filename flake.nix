{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in {
        devShells.default = import ./nix/devshell.nix {inherit pkgs rustToolchain;};
        packages.default = import ./nix/package.nix {inherit pkgs rustPlatform;};
      };
    };
}
