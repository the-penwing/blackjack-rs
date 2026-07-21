{
  description = "Rust devshell for blackjack-rs (optimised for my thinkpad)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [(import rust-overlay)];
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        # CORE UTILS FOR EASIER DEVELOPMENT
        git
        starship
        jq
        ripgrep
        fd
        bat
        eza

        # RUST OVERLAY
        (rust-bin.stable.latest.default.override {
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
        })

        # TOOLSET FOR CROSS-COMPILING WITH ZIG
        gcc
        cargo-zigbuild
        zig

        # PERFORMANCE TOOLING
        mold
        clang
      ];
      shellHook = ''
        echo "Rust DevShell Loaded"
        echo "  Rust: $(cargo --version)"
        echo "  GCC: $(gcc --version | head -n1)"
        echo "  Zig: $(zig version)"
        echo "  Linker: mold $(mold --version | head -n1)"

        # PERFORMACE FLAGS FOR MY THINKPAD P14s GEN 2
        export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold -C target-cpu=native"
        export CARGO_TARGET_DIR="/tmp/cargo-target"

        alias ls='eza'
        alias ll='eza --icons -l'
        alias la='eza --icons -la'

        eval "$(${pkgs.starship}/bin/starship init bash)"
      '';
    };
  };
}
