{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShells.${system} = {
      default = pkgs.mkShell {
        name = "euchre";
        buildInputs = with pkgs; [
          just

          # python
          uv

          # typst
          typst
          tinymist

          # rust
          cargo
          rustfmt
          cargo-flamegraph # profiling?
          
          # required for numpy
          stdenv.cc.cc
          zlib
        ];

        shellHook = ''
          just --list
        '';

        # required for numpy
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc pkgs.zlib ];
      };
    };
  };
}
