{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.illuminator = naersk-lib.buildPackage {
        pname = "illuminator";
        root = ./.;
      };
      packages.default = packages.illuminator;

      apps.illuminator = utils.lib.mkApp {
        drv = packages.illuminator;
      };
      apps.default = apps.illuminator;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
