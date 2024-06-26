# just manual: https://github.com/casey/just/#readme

_default:
  @just --list

# Installs the trunk packager
install-trunk:
  cargo install trunk

# Run the web server
run-web: web-app
  cargo run --bin clean-architecture-with-rust-web

# Run the CLI
run-cli:
  cargo run --bin clean-architecture-with-rust-cli

# Run the desktop app
run-desktop:
  cargo run --bin clean-architecture-with-rust-desktop

# Build the web server
build-web: web-app
  cargo build --bin clean-architecture-with-rust-web --release

# Build the CLI
build-cli:
  cargo build --bin clean-architecture-with-rust-cli --release

# Build the desktop app
build-desktop:
  cargo build --bin clean-architecture-with-rust-desktop --release

# Build the web app
web-app:
  cd crates/web-app/ && trunk build

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
  cd crates/web-app/ && cargo fmt --all

# Run clippy linter
clippy:
  cargo clippy --workspace --fix --allow-dirty --allow-staged
  cargo fix --workspace --allow-dirty --allow-staged
  cd crates/web-app/ && cargo clippy --workspace --fix --allow-dirty --allow-staged
  cd crates/web-app/ && cargo fix --workspace --allow-dirty --allow-staged

# Fix lint warnings
fix:
  cargo fix --workspace --all-targets
  cargo clippy --workspace --all-targets --fix
  cd crates/web-app && cargo fix --workspace --all-targets
  cd crates/web-app && cargo clippy --workspace --all-targets --fix

# Run tests
test:
  RUST_BACKTRACE=1 cargo test --locked --workspace -- --nocapture
  RUST_BACKTRACE=1 cargo test --locked --workspace --manifest-path crates/web-app/Cargo.toml -- --nocapture
  RUST_BACKTRACE=1 wasm-pack test --chrome --headless crates/web-app/

# Upgrade (and update) dependencies and tools
upgrade:
  cargo upgrade --incompatible
  cargo update
  cd crates/web-app && cargo upgrade --incompatible
  cd crates/web-app && cargo update
