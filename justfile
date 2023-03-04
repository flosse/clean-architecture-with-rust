# just manual: https://github.com/casey/just/#readme

_default:
  @just --list

# Set up (and update) tooling
setup:
  cargo install trunk

# Run the web server
run-web: web-app-seed
  cargo run --bin clean-architecture-with-rust-web

# Run the CLI
run-cli:
  cargo run --bin clean-architecture-with-rust-cli

# Run the desktop app
run-desktop:
  cargo run --bin clean-architecture-with-rust-desktop

# Build the web server
build-web: web-app-seed
  cargo build --bin clean-architecture-with-rust-web --release

# Build the CLI
build-cli:
  cargo build --bin clean-architecture-with-rust-cli --release

# Build the desktop app
build-desktop:
  cargo build --bin clean-architecture-with-rust-desktop --release

# Build the web app
web-app-seed:
  cd crates/web-app-seed/ && trunk build

# Read version from Cargo.toml
pkg-version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

# Create a tarball with the webserver
pack-web: build-web
  tar -C target/release/ \
      -cvpJf clean-architecture-with-rust-web_{{pkg-version}}.tar.xz \
      clean-architecture-with-rust-web

# Create a tarball with the CLI
pack-cli: build-cli
  tar -C target/release/ \
      -cvpJf clean-architecture-with-rust-cli_{{pkg-version}}.tar.xz \
      clean-architecture-with-rust-cli

# Format source code
fmt:
  cargo fmt --all
  cd crates/web-app-seed/ && cargo fmt --all

# Run clippy linter
clippy:
  cargo clippy --workspace
  cd crates/web-app-seed/ && cargo clippy --workspace

# Fix lint warnings
fix:
  cargo fix --workspace --all-targets
  cargo clippy --workspace --all-targets --fix
  cd crates/web-app-seed && cargo fix --workspace --all-targets
  cd crates/web-app-seed && cargo clippy --workspace --all-targets --fix

# Run tests
test:
  RUST_BACKTRACE=1 cargo test --locked --workspace -- --nocapture
  RUST_BACKTRACE=1 cargo test --locked --workspace --manifest-path crates/web-app-seed/Cargo.toml -- --nocapture
  RUST_BACKTRACE=1 wasm-pack test --chrome --headless crates/web-app-seed/
