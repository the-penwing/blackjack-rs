{
  pkgs,
  rustToolchain,
}:
pkgs.mkShell {
  nativeBuildInputs = [pkgs.pkg-config rustToolchain];
  name = "blackjack-rs";
  packages = with pkgs; [
    just
  ];
}
