let
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust";
    buildInputs = [ 
      clippy
      openssl pkg-config rustup rustracer cargo-flamegraph cmake zlib ];
  }
