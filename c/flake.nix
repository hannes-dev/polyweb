{
  description = "C webserver";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          name = "c-web-server";
          version = "0.1.0";
          src = self;

          nativeBuildInputs = [
            pkgs.clang
          ];

          buildPhase = ''
            clang src/hello.c -o c-web-server
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp c-web-server $out/bin/
          '';
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.clang        
            pkgs.gnumake       
            pkgs.gdb        
            pkgs.clang-tools
            pkgs.valgrind
          ];
        };
      }
    );
}
