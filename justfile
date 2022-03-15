# Disable dotenv support to avoid automatically loading from '.env' files
set dotenv-load := false
set positional-arguments := true

export RUST_BACKTRACE := "1"

_default:
    @just --list

# Build the project with the `tokio_unstable` flag set
build:
    cargo build

# Build the release version
build-release:
    cargo build --release

# Format, lint and check that project compiles
compile:
    cargo fmt --all
    cargo clippy -- -D warnings

# Format the project with rustfmt
format:
    cargo fix
    cargo clippy --fix
    cargo fmt --all

# Run basic formatter and Clippy linter
lint:
    cargo fmt --all
    cargo clippy

# Run code-quality and CI-related tasks locally
pre-commit:
    cargo fmt --all -- --check
    cargo clippy -- --D warnings
    cargo test

# Run tests without capturing I/O
test:
    cargo test -- --nocapture

# Run all tests sequentially without capturing I/O
test-debug:
    cargo test -- --test-threads=1 --nocapture
