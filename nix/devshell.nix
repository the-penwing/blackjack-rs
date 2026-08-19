{
  pkgs,
  rustToolchain,
}:
pkgs.mkShell {
  nativeBuildInputs = [pkgs.pkg-config rustToolchain];
}
