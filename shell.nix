let
  src = builtins.fetchTarball {
    # nixpkgs-19.03 as of 2019/04/15.
    url = "https://github.com/NixOS/nixpkgs/archive/dfd8f84aef129f1978e446b5d45ef05cd4421821.tar.gz";
    sha256 = "sha256:0av0q7xyv76jq2csbg10x8gcnlnadlppvlx616s7qz7jahkmymrl";
  };
in

with import src {};

stdenv.mkDerivation {
  name = "exercism-rust-env";
  nativeBuildInputs = [
    cargo
    rustc
    rustfmt
    rustup

    # Example Build-time Additional Dependencies
    pkgconfig
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    openssl
  ];

  # Set Environment Variables
  #RUST_BACKTRACE = 1;
}
