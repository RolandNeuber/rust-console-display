{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ fenix.overlays.default ];
      };
      toolchain = pkgs.fenix.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-VYaaz/Iz3wita53RiO4c00Orb/D5BQ9w66sjGd+QT+k=";
      };
      cargo-coupling = pkgs.rustPlatform.buildRustPackage {
        pname = "cargo-coupling";
        version = "v0.3.7";

        nativeBuildInputs = with pkgs; [
          git
        ];
        
        src = pkgs.fetchFromGitHub {
          owner = "nwiizo";
          repo = "cargo-coupling";
          rev = "main";
          hash = "sha256-lIgtOVCgUiB319RwNF1G3UHCX7dl65F3VTnF8gLe8sE=";
        };

        cargoHash = "sha256-UQCneddJgqziw+vaSLKkaOC+hthjQzYyLWMZTsnsKVc=";
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          toolchain
          pkg-config
          ffmpeg

          clang
          llvmPackages.libclang

          cargo-modules
          cargo-expand
          cargo-coupling
        ];

        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.ffmpeg
        ];
      };
    };
}
