{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell =
          with pkgs;
          mkShell.override { stdenv = pkgs.clangStdenv; } rec {
            nativeBuildInputs = [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
              pkg-config
              pre-commit
              git-cliff
              cargo-release
              cargo-outdated
            ];
            buildInputs = [
              speechd
              udev
              alsa-lib
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              libxkbcommon
              wayland
            ];
            shellHook = ''
              export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
              export RUSTFLAGS="-C link-arg=-Wl,-rpath,${lib.makeLibraryPath buildInputs}"
              pre-commit install
            '';
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
