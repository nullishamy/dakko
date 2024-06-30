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
        { pkgs, ... }:
        {
          default =
            let
              libPath =
                with pkgs;
                lib.makeLibraryPath [
                  libGL
                  libxkbcommon
                  wayland
                  xorg.libX11
                  xorg.libXcursor
                  xorg.libXi
                  xorg.libXrandr
                ];
            in
            # ...
            pkgs.mkShell {
              env = {
                LD_LIBRARY_PATH = libPath;
              };
              packages = with pkgs; [
                xorg.libxcb
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
    };
}
