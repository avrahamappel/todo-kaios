{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
  buildInputs = [
    pkgs.androidenv.androidPkgs_9_0.platform-tools
    pkgs.cargo
    # pkgs.firefox
    pkgs.trunk
  ];
}
