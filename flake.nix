{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in
      {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              rustVersion
              clippy
              cargo-deny
              cargo-edit
              cargo-watch
              cargo-nextest
              typos
              just
            ];

            # Set Environment Variables
            # RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${rustVersion}/lib/rustlib/src/rust/src";

            TOOLCHAIN_LOCATION = "${rustVersion}/bin";
            STANDARD_LIBRARY = "${rustVersion}/lib/rustlib/src/rust";
            # Toolchain location:
            # printf "$TOOLCHAIN_LOCATION" | pbcopy
            # Standard library:
            # printf "$STANDARD_LIBRARY" | pbcopy
          };
      });
}
