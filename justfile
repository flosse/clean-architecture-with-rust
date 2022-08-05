# just manual: https://github.com/casey/just/#readme

_default:
  @just --list

# Set up (and update) tooling
setup:
  cargo install trunk

run: webapp
  cargo run -- serve

build: webapp
  cargo build --release

# Build the web app
webapp:
  cd webapp/ && trunk build

# Read version from Cargo.toml
pkg-version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

# Create a tarball
pack: build
  tar -C target/release/ \
      -cvpJf clean-architecture-with-rust_{{pkg-version}}.tar.xz \
      clean-architecture-with-rust

# Format source code
fmt:
  cargo fmt --all
  cd webapp/ && cargo fmt --all

# Fix lint warnings
fix:
  cargo fix --workspace --all-targets
  cargo clippy --workspace --all-targets --fix
  cd webapp && cargo fix --workspace --all-targets
  cd webapp && cargo clippy --workspace --all-targets --fix
