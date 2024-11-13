{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [
    act
    git
  ];

  languages = {
    python = {
      version = "3.12";
      enable = true;
      poetry = {
        enable = true;
      };
    };
  };
  languages.rust.enable = true;
}
