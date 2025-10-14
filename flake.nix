{
  description = "Palet - A Rust GTK application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          rust-analyzer
          pkg-config
          wrapGAppsHook4
        ];

        buildInputs = with pkgs; [
          direnv
          openssl
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          
          shellHook = ''
            export RUST_BACKTRACE=1
            eval "$(direnv hook bash)"
            direnv allow
          '';

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [];
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "plexm3u";
          version = "0.1.0";
          
          src = ./.;
          
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
        
        packages.palet = self.packages.${system}.default;
      });
}
