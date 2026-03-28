{
  description = "Plexm3u";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
          ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          rust-analyzer
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "plexm3u";

          src = ./.;
          cargoBuildOptions = [ "--release" ];
          buildTarget = "x86_64-unknown-linux-musl";

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          meta = with pkgs.lib; {
            description = "";
            homepage = "https://github.com/nicklayb/plexm3u";
            license = licenses.mit;
            maintainers = [ ];
            platforms = platforms.linux;
            mainProgram = "plexm3u";
          };
        };
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs;

          buildInputs = buildInputs ++ [
            pkgs.direnv
            pkgs.just
          ];

          shellHook = ''
            export RUST_BACKTRACE=1
            eval "$(direnv hook bash)"
            direnv allow
          '';

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ ];
        };

        packages.default = defaultPackage;

        packages.container = pkgs.dockerTools.buildLayeredImage {
          name = "plexm3u";
          tag = "latest";
          created = "now";
          contents = [ defaultPackage ];
          config = {
            Entrypoint = [ "${defaultPackage}/bin/plexm3u" ];
            Env = [
              "PATH=${defaultPackage}/bin:$PATH"
            ];
          };
        };

        packages.plexm3u = self.packages.${system}.default;
      }
    );
}
