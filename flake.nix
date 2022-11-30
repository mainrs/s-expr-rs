{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix";
    flake-compat = {
      flake = false;
      url = "github:edolstra/flake-compat";
    };
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustChannel = "nightly";
          packageFun = import ./Cargo.nix;
          extraRustComponents = [ "clippy" "rustfmt" ];
        };

        workspaceShell = rustPkgs.workspaceShell {
          packages = with pkgs; [
            cargo-audit
            cargo-auditable
            cargo-edit
            cargo-expand
          ];

          shellHook = ''
            export CARGO_UNSTABLE_SPARSE_REGISTRY=true
          '';
        };
      in rec {
        devShell = workspaceShell;
        packages = {
          
        };
      }
    );
}