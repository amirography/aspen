{
  description = "nixos helper for this nixos user"
  ;
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:

    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };
        buildInputs = with pkgs; [
          openssl
        ];

        nativeBuildInputs = with pkgs; [ pkg-config ];
      in
      {

        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          name = "nixme";
          version = "0.0.1";

          inherit buildInputs nativeBuildInputs;


        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
        };
      }
    );
}
