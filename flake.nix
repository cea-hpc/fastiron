{
  description = "Nix flake for Rust project with hwloc";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustToolchain
            cargo
            rust-analyzer
            pkg-config
          ];
          buildInputs = with pkgs; [
            hwloc.dev # Provides hwloc library
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.hwloc.lib}/lib:$LD_LIBRARY_PATH
          '';
        };
      });
}
