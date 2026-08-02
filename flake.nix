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
      perSystem = {
        system,
        pkgs,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };
        rustToolchain = pkgs.rust-bin.stable."1.97.1".default.override {
          targets = ["i686-unknown-linux-musl"];
        };
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          nativeBuildInputs = with pkgs; [clang mold];
          RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=mold";
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        ishArgs =
          commonArgs
          // {
            CARGO_BUILD_TARGET = "i686-unknown-linux-musl";
            CARGO_TARGET_I686_UNKNOWN_LINUX_MUSL_RUSTFLAGS = "-C target-cpu=pentium4";
            RUSTFLAGS = "";
            nativeBuildInputs = with pkgs; [cargo-zigbuild zig];
            cargoExtraArgs = "--target i686-unknown-linux-musl";
            cargoBuildCommand = "cargo zigbuild";
            preBuild = "export HOME=$TMPDIR";
          };

        ishArtifacts = craneLib.buildDepsOnly ishArgs;
      in {
        packages.default = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});
        packages.ish = craneLib.buildPackage (ishArgs // {cargoArtifacts = ishArtifacts;});
        devShells.default = craneLib.devShell {
          packages = with pkgs; [just cargo-zigbuild zig];
          shellHook = ''
            export CARGO_TARGET_I686_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-cpu=pentium4"
          '';
        };
      };
    };
}
