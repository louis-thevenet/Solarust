{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };
  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem = {
        config,
        self',
        pkgs,
        lib,
        system,
        ...
      }: let
        fenixPkgs = inputs.fenix.packages.${system};
        toolchain = fenixPkgs.combine [
          # Default Rust tools
          fenixPkgs.stable.cargo
          fenixPkgs.stable.clippy
          fenixPkgs.stable.rust-src
          fenixPkgs.stable.rustc
          fenixPkgs.stable.rustfmt
        ];

        nativeBuldInputs = [pkgs.pkg-config];

        buildInputs = with pkgs; [
          udev
          alsa-lib
          vulkan-loader
          libxkbcommon
          wayland
        ];
      in {
        # Rust dev environment
        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

          RUST_BACKTRACE = "full";
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

          packages =
            buildInputs
            ++ [toolchain]
            ++ nativeBuldInputs
            ++ [
              # Nix tools
              pkgs.nil
              pkgs.alejandra
            ];
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
    };
}
