{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.mozilla-overlay.url = "github:mozilla/nixpkgs-mozilla";

  outputs = { self, nixpkgs, flake-utils, mozilla-overlay }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      apple = pkgs.darwin.apple_sdk.frameworks;
      apple-deps = [ apple.Security pkgs.libiconv ];
    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs = [ pkgs.bashInteractive ];
        buildInputs = [ 
          pkgs.pkg-config
          pkgs.protobuf
          pkgs.openssl
          pkgs.cargo
          pkgs.rust-analyzer
        ] ++ (if system == "x86_64-darwin" then apple-deps else []);
      };
    });
}
