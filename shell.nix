let arsenal = builtins.fetchGit {
  url = "https://github.com/hackbg/arsenal";
  rev = "f047c0f3009553561f822269098de065b0d46ab9";
}; in import "${arsenal.outPath}/shells/rust.nix" rec {

  pkgs = import <nixpkgs> {
    overlays = import "${arsenal.outPath}/overlays.nix";
  };

  extraBuildInputs = with pkgs; [
    libclang
    eudev
  ];

  extraNativeBuildInputs = with pkgs; [
    pkg-config
    protobuf
  ];

}
