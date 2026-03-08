{
  description = "Apple Mail.app integration via AppleScript";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "apple-mail";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        openclawPlugin = {
          name = "apple-mail";
          skills = [ ./skills/apple-mail ];
          packages = [ self.packages.${system}.default ];
          needs = {
            stateDirs = [];
            requiredEnv = [];
          };
        };
      }
    );
}
