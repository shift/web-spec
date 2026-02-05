{
  description = "web-spec - Gherkin Feature Runner for browser automation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Build dependencies (libraries needed for linking)
        buildDeps = with pkgs; [
            openssl
            pkg-config
        ];

        # Runtime dependencies (browsers)
        runtimeDeps = with pkgs; [
            chromium
            chromedriver
        ];

        # Build the package using cargo2nix
        packages.default = pkgs.rustPlatform.packages.rustPlatform.cargo2nix.buildPackage {
          src = ./.;
          inherit pkgs buildDeps runtimeDeps;
        };
      in
      {
        # 1. The Package (nix build)
        packages.default = pkgs.rustPlatform.packages.rustPlatform.cargo2nix.buildPackage {
          src = ./.;
          inherit pkgs buildDeps runtimeDeps;

          # Fix for unit tests that might need a browser
          postInstall = ''
            wrapProgram $out/bin/web-spec \
              --prefix PATH : ${pkgs.lib.makeBinPath runtimeDeps}
          '';
        };

        # 2. The Dev Shell (nix develop)
        devShells.default = pkgs.mkShell {
          buildInputs = buildDeps ++ runtimeDeps ++ [
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
          ];

          # Explicitly set OpenSSL dir for pkg-config
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          # Set Rust source path for IDEs
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
