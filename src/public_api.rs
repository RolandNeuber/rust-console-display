#[test]
fn public_api() {
    use std::fs::read_to_string;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Toolchain {
        channel: String
    }

    let Toolchain { channel } = toml::from_str::<Toolchain>(&read_to_string("rust-toolchain.toml").unwrap()).unwrap();

    // Install a compatible nightly toolchain if it is missing.
    rustup_toolchain::install(&channel).unwrap();

    // Build rustdoc JSON.
    let rustdoc_json = rustdoc_json::Builder::default()
        .toolchain(&channel)
        .build()
        .unwrap();

    // Derive the public API from rustdoc JSON.
    let public_api = public_api::Builder::from_rustdoc_json(rustdoc_json)
        .build()
        .unwrap();

    // Assert that the public API matches the latest snapshot.
    // Run with env var `UPDATE_SNAPSHOTS=yes` to update the snapshot.
    public_api.assert_eq_or_update("./tests/snapshots/public-api.txt");
}