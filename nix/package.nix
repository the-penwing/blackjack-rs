{
  pkgs,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "blackjack-rs";
  version = "0.1.0";

  src = pkgs.lib.fileset.toSource {
    root = ../.;
    fileset = pkgs.lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      ../src
    ];
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [pkgs.pkg-config pkgs.makeWrapper];

  meta = {
    description = "Blackjack for the terminal - Written in Rust";
    homepage = "https://github.com/the-penwing/blackjack-rs";
    license = pkgs.lib.licenses.agpl3Only;
    maintainers = [
      {
        name = "Ben van Leeuwen";
        email = "benvanleeuwen01@gmail.com";
        github = "the-penwing";
      }
    ];
  };
}
