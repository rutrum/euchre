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
          uv
          just
          typst
          tinymist
          
          # required for numpy
          stdenv.cc.cc
          zlib
        ];

        # required for numpy
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc pkgs.zlib ];
      };
    };
  };
}
