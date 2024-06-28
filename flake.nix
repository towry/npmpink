{
  description = "rust-workspace template";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=24.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = {
    self,
    nixpkgs,
    fenix,
    naersk,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (_: super: let pkgs' = fenix.inputs.nixpkgs.legacyPackages.${super.system}; in fenix.overlays.default pkgs' pkgs')
        ];
      };
      supportedPlatforms = {
        aarch64-darwin = {
          rustTarget = "aarch64-apple-darwin";
        };
        aarch64-linux = {
          rustTarget = "aarch64-unknown-linux-gnu";
        };
        x86_64-darwin = {
          rustTarget = "x86_64-apple-darwin";
        };
        x86_64-linux = {
          rustTarget = "x86_64-unknown-linux-gnu";
        };
      };
      rustTarget = supportedPlatforms.${system}.rustTarget;
      apple_sdk = pkgs.darwin.apple_sdk.frameworks;
      rust-toolchain = with pkgs.fenix;
        combine [
          stable.cargo
          stable.rustc
          stable.rustfmt
          stable.clippy
          targets.${rustTarget}.stable.rust-std
          # targets.wasm32-wasi.stable.rust-std
        ];
      naersk' = pkgs.callPackage naersk {
        cargo = rust-toolchain;
        rustc = rust-toolchain;
      };
      buildInputs =
        [pkgs.openssl]
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv apple_sdk.DiskArbitration apple_sdk.Foundation apple_sdk.Security apple_sdk.SystemConfiguration];
    in {
      packages.default = naersk'.buildPackage {
        src = ./.;
        buildInputs = buildInputs;
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
      };
      devShells.default = pkgs.mkShell {
        buildInputs = buildInputs;
        nativeBuildInputs = [pkgs.rust-analyzer-nightly rust-toolchain pkgs.pkg-config];
      };
    });
}
