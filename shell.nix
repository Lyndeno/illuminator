{ pkgs ? import<nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ sqlite diesel-cli rustup clang];
  }
