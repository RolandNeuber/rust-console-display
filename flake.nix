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
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          toolchain
          pkg-config
          ffmpeg

          clang
          llvmPackages.libclang
        ];

        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.ffmpeg
        ];
      };
    };
}
