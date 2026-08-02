{
  description = "Rust devshell and build system for blackjack-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        rustToolchain = pkgs.rust-bin.stable."1.97.1".default.override {
          extensions = ["rust-src" "rust-analyzer"];
          targets = [
            "i686-unknown-linux-musl"
            "aarch64-unknown-linux-gnu"
            "aarch64-unknown-linux-musl"
            "x86_64-pc-windows-gnu"
            "aarch64-apple-darwin"
            "x86_64-apple-darwin"
            "x86_64-unknown-linux-gnu"
            "x86_64-unknown-linux-musl"
          ];
        };

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [clang mold gcc];
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=mold -C target-cpu=native";
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in {
        packages.default = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});

        devShells.default = craneLib.devShell {
          packages = with pkgs; [git lazygit just ripgrep bat eza cargo-zigbuild zig];

          shellHook = ''
            export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold -C target-cpu=native"
            export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-cpu=x86-64-v3"
            export CARGO_TARGET_I686_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-cpu=pentium4"
          '';
        };
      };
    };
}
