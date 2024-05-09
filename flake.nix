{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  description = "Development shell flake";
  outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachDefaultSystem (system: {
    devShell = let 
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
          inherit system overlays;
      };

      libraries = with pkgs; [
        webkitgtk
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        openssl_3
        librsvg
      ];

      packages = with pkgs; [
        curl
        wget
        pkg-config
        dbus
        openssl_3
        glib
        gtk3
        libsoup
        webkitgtk
        librsvg
      ];
    in 
      pkgs.mkShell {
         buildInputs = packages;

        shellHook =
          ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
            export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
          '';
        packages = with pkgs; [
          nodejs_20
          nodePackages.npm
          (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          })
        ];
      };
  });
}
