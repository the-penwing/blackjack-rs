# list recipes
default:
  just --list

# run the CLI inside the nix shell
run:
  nix develop -c cargo run

# run tests inside the nix shell
test:
  nix develop -c cargo test

# clippy
clippy:
  nix develop -c cargo clippy -- -D warnings

# format
fmt:
  nix develop -c cargo fmt

# check (format -> clippy -> test)
check: fmt clippy test

# build release binary
build:
 nix develop -c cargo build --release
