{
  description = "Axum web service (Rust)";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Stable Rust + clippy + rustfmt + rust-analyzer
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "axum-dev";

          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer

            # OpenSSL (system) – headers + libs
            openssl.dev
            openssl.out
            pkg-config

            #AWS
            awscli2

            # Conveniences
            cargo-watch
            cargo-edit
            cargo-outdated
            just
            git
          ];

          # Force openssl-sys to use Nix OpenSSL
          OPENSSL_NO_VENDOR = 1;

          # Make sqlx (or any other ~/.cargo/bin tools) available
          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
            echo "Axum dev-shell ready. Happy hacking!"

          '';
        };
      });
}