{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self,nixpkgs,fenix}:

        let
        system = "x86_64-linux";
        pkgs = import nixpkgs { system = system; };
        fenixPkgs = fenix.packages.${system};
   toolchain = fenixPkgs.combine [
          # Default Rust tools
          fenixPkgs.stable.cargo
          fenixPkgs.stable.clippy
          fenixPkgs.stable.rust-src
          fenixPkgs.stable.rustc
          fenixPkgs.stable.rustfmt
          # Needed by engine WASM build
          fenixPkgs.targets.wasm32-unknown-unknown.stable.rust-std
        ];

nativeBuldInputs = [pkgs.pkg-config];

          buildInputs = with pkgs; [
           udev alsa-lib vulkan-loader
            libxkbcommon wayland
          ];

          wasmBuildInputs = [
          pkgs.wasm-pack
          pkgs.wasm-bindgen-cli
        ];

        in
{
          # Rust dev environment
          devShells."x86_64-linux".default = pkgs.mkShell {

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

            RUST_BACKTRACE = "full";
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

            packages = buildInputs ++ [toolchain]  ++ wasmBuildInputs ++ nativeBuldInputs;
            shellHook=''export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner'';
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };
        };
    }