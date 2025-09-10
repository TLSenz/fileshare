{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.openssl
        pkgs.pkg-config
        pkgs.sqlx-cli
        pkgs.tokei
      ];

      # Correct way to set environment variables
      shellHook = ''
        export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig
      '';
    };
  };
}
