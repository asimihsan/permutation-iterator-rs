#!/usr/bin/env bash

set -euxo pipefail

cargo check
cargo check --target wasm32-unknown-unknown
cargo check --target wasm32-unknown-emscripten
cargo check --target wasm32-wasi

rustup run nightly cargo clippy -- -D warnings
cargo test --release
rustup run nightly cargo bench
# cargo test --test randomness --release -- --nocapture --ignored

