{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    astal-niri = {
      url = "github:sameoldlab/astal/feat/niri";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
      astal-niri,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        lib = pkgs.lib;
        astal-deps =
          (with pkgs.astal; [
            io
            hyprland
            bluetooth
            river
            battery
            network
            mpris
            tray
            astal4
            wireplumber
          ])
          ++ [
            (astal-niri.packages.${system}.niri.overrideAttrs {
              nativeBuildInputs = with pkgs; [
                wrapGAppsHook4
                gobject-introspection
                meson
                pkg-config
                ninja
                vala
                wayland
                wayland-scanner
                python3
              ];
            })
          ];
        gtk-deps = with pkgs; [
          glib.dev
          gtk4.dev
          gobject-introspection
          gtk3
          cairo
          pango
          gdk-pixbuf
          graphene
          gtk4-layer-shell
          freetype
          wayland.dev
        ];
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = pkgs.mkShell {
          buildInputs =
            (with pkgs; [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              pkg-config
              gir-rs
              nushell
              xmlstarlet
            ])
            ++ astal-deps
            ++ gtk-deps;

          LD_LIBRARY_PATH = lib.makeLibraryPath (astal-deps ++ gtk-deps);

          GIR_DIRS = lib.makeSearchPathOutput "dev" "share/gir-1.0" astal-deps;

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
