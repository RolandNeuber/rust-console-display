cargo fmt --all
RUSTFLAGS="-Dclippy::all" cargo clippy --all-targets --all-features --fix --allow-dirty --broken-code
