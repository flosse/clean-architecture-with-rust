let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rustChannel = pkgs.rustChannelOf {
     channel = "stable";
  };
  rust = (rustChannel.rust.override {
    targets = [
      "wasm32-unknown-unknown" # required for the webapp
    ];
  });

  # required for the desktop
  runtime_deps = with pkgs; [
      libGL
  ] ++ (with pkgs.xorg; [
      libX11
      libXcursor
      libXrandr
      libXi
  ]);

in
  with pkgs;
  mkShell {
    buildInputs = [

      rust
      pkgconfig

      # required for the webapp
      sassc

      # required for the desktop
      freetype
      expat
      fontconfig
    ];

    LD_LIBRARY_PATH = "${lib.makeLibraryPath runtime_deps}";
}
