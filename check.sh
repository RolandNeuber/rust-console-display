cargo +nightly fmt --all --check
RUSTFLAGS="-Dclippy::all" cargo +nightly clippy --all-targets --all-features
cargo test