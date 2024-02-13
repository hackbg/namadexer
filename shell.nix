let
  rev   = "a5da8f82aadf0b90175eace3ed96319c6dd9d5dd";
  url   = "https://raw.githubusercontent.com/hackbg/arsenal/${rev}/shells/rust-protobuf.nix";
  pkgs  = builtins.fetchGit {
    url = "https://github.com/hackbg/arsenal";
    ref = "main";
    rev = rev;
  };
in import (builtins.fetchurl url) {
  pkgs = import <nixpkgs> {
    overlays = import "${pkgs.outPath}/overlays.nix";
  };
}
