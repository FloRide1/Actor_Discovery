with import (builtins.fetchTarball {
  url =
    "https://github.com/NixOS/nixpkgs/archive/0cf4274b5d06325bd16dbf879a30981bc283e58a.tar.gz";
  sha256 = "0402hxwri0xnhwbcviwbv4xpy8wr2wcprvi62b26y2qf6kb8kyx2";
}) { };

mkShell {

  buildInputs =
    [ openssl pkg-config binutils cmake fontconfig rust-analyzer diesel-cli ];
}
