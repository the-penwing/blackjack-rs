# blackjack-rs

Simple blackjack game written in rust.

## Playing the Game

There are 3 ways to install / play the game

- [Cargo](#cargo)
- [Nix](#nix)

### Cargo

Requirements

- Cargo + Rust
- Git

```bash
git clone https://github.com/the-penwing/blackjack-rs
cd blackjack-rs
```

To run without installing

```bash
cargo run
```

To install

```bash
cargo install
```

### Nix

There are 3 ways to run via Nix - Profiles, Flakes and Run. Choose whichever you prefer.

**Install Via Profile (Single User)**

```bash
nix profile add github:the-penwing/blackjack-rs
```

```bash
blackjack-rs
```

**Using the Flake**

1. Add the game to your `flake.nix` inputs:

```nix
inputs = {
  nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  blackjack-rs.url = "github:the-penwing/blackjack-rs";
};
```

2. Pass `inputs.blackjack-rs` to your outputs and add the package to your system or or home-manager profile:

```nix
outputs = { self, nixpkgs, blackjack-rs, ... }: {
  # Example Flake Config
  nixosConfigurations."HOSTNAME" = nixpkgs.lib.nixosSystem {
    inherit system;
    modules = [
      ({ pkgs, ... }: {
        enviroment.systemPackages = [
          blackjack-rs.packages.${system}.default
        ];
      })
    ];
  };
};
```

**Run Without Installation (nix run)**

```bash
nix run github:the-penwing/blackjack-rs
```
