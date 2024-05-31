{
  inputs = {
    nixpkgs = {
      type = "indirect";
      id = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";

    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    naersk,
    fenix,
  }:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {
        inputs',
        config,
        system,
        pkgs,
        ...
      }: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain = fenix.packages."${system}".fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-GJ6T8aniCNJiznb/n/9l0lItNWinUk93FpdrNijxDog=";
        };

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };
      in {
        packages.default = naersk'.buildPackage ./.;

        formatter = pkgs.alejandra;

        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              toolchain
              cargo-expand

              pkgs.linuxPackages_latest.perf
            ];
            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          };
      };
    };
}
