{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, naersk, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        naersk' = pkgs.callPackage naersk {
          rustc = pkgs.rust-bin.nightly.latest.minimal;
          cargo = pkgs.rust-bin.nightly.latest.minimal;
        };

        greetd_stub = pkgs.rustPlatform.buildRustPackage {
          name = "greetd_stub";
          version = "0.3.0";

          src = pkgs.fetchFromGitHub {
            owner = "apognu";
            repo = "greetd-stub";
            rev = "186d4d100893c8dee94d6df3500995589f7f1037";
            sha256 = "sha256-oSW103PFuekgpSKz3ejE9WrGdSeSezdF8P3qQgiSus8=";
          };

          cargoHash = "sha256-e77Zd69DcKAnJEaJdOeCzDJAjysI+4QGJ3rjD+Y5SsI=";
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # For ballad-greeter development
            greetd_stub
            pkgs.zbus-xmlgen

            pkgs.brightnessctl
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config

            gtk4
            gtk4-layer-shell
            glib
            librsvg
            cairo

            libpulseaudio

            libxkbcommon
          ];
          LD_LIBRARY_PATH = with pkgs;
            pkgs.lib.makeLibraryPath [
              gtk4
              gtk4-layer-shell
              glib
              librsvg
              cairo

              libpulseaudio

              libxkbcommon
            ];
        };
        packages = rec {
          ballad = naersk'.buildPackage {
            pname = "ballad";
            name = "ballad";
            version = "0.1.0";
            src = ./.;

            nativeBuildInputs = with pkgs; [
              pkg-config

              gtk4
              gtk4-layer-shell
              glib
              librsvg
              cairo

              libpulseaudio

              libxkbcommon

              wrapGAppsHook4
            ];

            buildInputs = with pkgs; [
              gtk4
              gtk4-layer-shell
              glib
              librsvg
              cairo

              alsa-lib
            ];
          };

          default = ballad;
        };
      });
}
