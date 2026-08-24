{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-shell.url = "github:RolandNeuber/nix-config?dir=shells/rust";
    cargo-coupling.url = "github:RolandNeuber/nix-config?dir=shells/cargo-coupling";
  };

  outputs = { self, nixpkgs, fenix, rust-shell, cargo-coupling }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShells.${system}.default = rust-shell.lib.mkDevShell {
        inherit system;
        toolchainFile = ./rust-toolchain.toml;
        toolchainHash = "sha256-VYaaz/Iz3wita53RiO4c00Orb/D5BQ9w66sjGd+QT+k=";
        extraPackages = with pkgs; [
          pkg-config
          ffmpeg_8

          clang
          llvmPackages.libclang

          cargo-modules
          cargo-expand
          cargo-coupling.packages.${system}.default
        ];
        extraShellAttrs = {
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.ffmpeg
          ];
        };
      };
    };
}
