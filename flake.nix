{
  inputs.nixpkgs.url = "github:nixos/nixpkgs";

  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    # lib = pkgs.lib;
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs_22
        bun
        ffmpeg
      ];
    };
  };
}
