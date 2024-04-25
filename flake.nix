{
  inputs = {
    nixpkgs = {
      type = "indirect";
      id = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
    # flake-parts = {
    #   type = "indirect";
    #   id = "flake-parts";
    # };

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
          # overlays = [(import nixpkgs-mozilla)];
        };
        # toolchain =
        #   (pkgs.rustChannelOf {
        #     rustToolchain = ./rust-toolchain.toml;
        #     sha256 = "";
        #   })
        #   .rust;
        toolchain = fenix.packages."${system}".fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-OqiwKj9BjgrI3ZUPIgGKn2t10vWTtNPsNg4E8QsQ7l8=";
        };
        # toolchain = with fenix.packages.${system};
        #   combine [
        #     minimal.rustc
        #     minimal.cargo
        #     targets.x86_64-unknown-linux-musl.latest.rust-std
        #   ];

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

      in {
        packages.default = naersk'.buildPackage ./.;

        # TODO: devshell treefmt-nix pre-commit-hooks-nix
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              toolchain

              # pre-commit
              # rustup
            ];
            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          };
      };
    };
}
