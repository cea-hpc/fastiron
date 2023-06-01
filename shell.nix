with import <nixpkgs> {}; {
  env = stdenv.mkDerivation {
    name = "thread_binder";
    buildInputs = [
      (pkgs.callPackage ./oldhwloc.nix {})
    ];
  };
}
