{
  description = "cpkg";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShell = pkgs.mkShell.override {stdenv = pkgs.gccStdenv;} rec {
          packages = with pkgs; [
            cargo
            cargo-deny
            cargo-mutants
            cargo-semver-checks
            cargo-tarpaulin
            clippy
            rustc
            rustfmt
          ];
        };
      }
    );
}
