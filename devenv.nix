{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [
    act
    cargo-bump
    git
    ruff
  ];

  env.NIX_LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
    stdenv.cc.cc
  ]);
  env.NIX_LD = lib.fileContents "${pkgs.stdenv.cc}/nix-support/dynamic-linker";

  languages = {
    python = {
      version = "3.13";
      enable = true;
      poetry = {
        enable = true;
      };
    };
  };
  languages.rust.enable = true;
}
