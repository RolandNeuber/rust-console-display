cargo fmt --all --check
RUSTFLAGS="-Dclippy::all" cargo clippy --all-targets --all-features
cargo test