
with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      rev = "9f35c4b09fd44a77227e79ff0c1b4b6a69dff533";
      sha256 = "18h0nvh55b5an4gmlgfbvwbyqj91bklf1zymis6lbdh75571qaz0";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    latest.rustChannels.nightly.rust
    rustfmt
    openssl
    pkg-config
  ];
  LIBRARY_PATH = "${openssl}/lib/";
  PKG_CONFIG_PATH = "${openssl}/lib/pkgconfig";
  RUST_BACKTRACE = 1;
}
