{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forEachSystem =
        function:
        nixpkgs.lib.genAttrs systems (
          system:
          let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs { inherit system overlays; };
          in
          function { inherit system pkgs; }
        );
    in
    {
      devShells = forEachSystem (
        { pkgs, system }:
        {
          default =
            let

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

              shellHook = ''
                export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
                export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
                export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
              '';
              packages = with pkgs; [
                nodejs_20
                nodePackages.yarn
                (rust-bin.stable.latest.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                    "clippy"
                  ];
                })
              ];
            };
        }
      );

      formatter = forEachSystem ({ pkgs, ... }: pkgs.nixfmt-rfc-style);

      packages = forEachSystem (
        { pkgs, system }:
        let
          pkg = pkgs.callPackage ./package.nix { };
        in
        {
          dakko = pkg;
          default = self.packages.${system}.dakko;
        }
      );
    };
}
