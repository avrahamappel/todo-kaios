{ pkgs ? import <nixpkgs> { } }:
let
  inherit (pkgs.stdenv.hostPlatform) isDarwin;
  kaiosNixEnv = pkgs.callPackage ./.nix { };
  firefox = pkgs.callPackage .nix/firefox.nix { };
  firefoxMac = pkgs.writeShellScriptBin "firefox" ''
    open ${firefox}/Applications/Firefox-KaiOS.app
  '';
in
pkgs.mkShell
{
  buildInputs = with pkgs; [
    binaryen
    nodejs
    trunk
    kaiosNixEnv.package
  ] ++ (if isDarwin then [ firefoxMac ] else [ ]);
}
