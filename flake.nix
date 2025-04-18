{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};

      # Rust toolchain with development tools
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = ["wasm32-unknown-unknown"];
        extensions = ["rust-src" "rust-analyzer" "clippy" "rustfmt"];
      };

      nodejs = pkgs.nodejs_20;
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustToolchain
          wasm-pack

          rust-analyzer

          nodejs
          bun

          gnumake
        ];

        shellHook = ''
          echo "Rust development environment ready!"
        '';
      };
    });
}
