# List recipes
default:
  just --list

# Run the CLI
run:
  cargo run

# Run tests
test:
  cargo test

# Run cargo clippy
clippy:
  cargo clippy -- -D warnings

# Format all files
fmt:
  cargo fmt
  alejandra .

# Check (format -> clippy -> test)
check: fmt clippy test

# Build release binary
build:
 cargo build --release
