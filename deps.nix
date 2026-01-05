{ pkgs, ... }:
let
  fmt = pkgs.fetchFromGitHub {
    owner = "fmtlib";
    repo = "fmt";
    rev = "407c905e45ad75fc29bf0f9bb7c5c2fd3475976f";
    sha256 = "sha256-ZmI1Dv0ZabPlxa02OpERI47jp7zFfjpeWCy1WyuPYZ0=";
  };
  raylib = pkgs.fetchFromGitHub {
    owner = "raysan5";
    repo = "raylib";
    rev = "1c3f9fa135cafa494af760c4cd0541c9a23cdf60";
    sha256 = "sha256-bJzuF/5UFIHewQFyCDDPrkh6eyEUv4AYAzmkkV/WjjI=";
  };
in
{
  setup_script = ''
    mkdir -p deps
    cp -r ${fmt} deps/fmt
    cp -r ${raylib} deps/raylib
  '';
}
