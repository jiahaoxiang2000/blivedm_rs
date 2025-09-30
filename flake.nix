{
  description = "blivedm_rs - Bilibili live room danmaku WebSocket client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Runtime dependencies
            openssl
            alsa-lib
            sqlite
            espeak-ng
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.AudioUnit
            pkgs.darwin.apple_sdk.frameworks.CoreAudio
            pkgs.darwin.apple_sdk.frameworks.Security
          ];

          shellHook = ''
            echo "🦀 Welcome to blivedm_rs development environment!"
            echo "Available commands:"
            echo "  nix run                        - Run the danmu binary"
            echo "  nix run . -- --room-id 24779526"
            echo "  nix run . -- --room-id 24779526 --tts-server http://localhost:8000"
          '';
        };

        packages.default =
          let
            # Select binary based on system
            binaryName = if pkgs.stdenv.isLinux then "danmu-linux-x86_64"
                        else if pkgs.stdenv.isDarwin && pkgs.stdenv.isAarch64 then "danmu-macos-arm64"
                        else if pkgs.stdenv.isDarwin then "danmu-macos-x86_64"
                        else throw "Unsupported platform";
          in
          pkgs.stdenv.mkDerivation rec {
            pname = "blivedm_rs";
            version = "0.4.0";

            src = pkgs.fetchurl {
              url = "https://github.com/jiahaoxiang2000/blivedm_rs/releases/download/v${version}/${binaryName}";
              sha256 = if pkgs.stdenv.isLinux then "11l706665ak56n9sr8a16p17krcscr6q76pn066cjm7vyi5wx4zq"
                      else if pkgs.stdenv.isDarwin && pkgs.stdenv.isAarch64 then "0000000000000000000000000000000000000000000000000000"  # Update with actual hash
                      else if pkgs.stdenv.isDarwin then "0000000000000000000000000000000000000000000000000000"  # Update with actual hash
                      else throw "Unsupported platform";
            };

            dontUnpack = true;
            dontBuild = true;

            nativeBuildInputs = with pkgs; [ autoPatchelfHook ];

            buildInputs = with pkgs; [
              # Runtime dependencies
              openssl
              alsa-lib
              sqlite
              espeak-ng
              pkgs.stdenv.cc.cc.lib
              pkgs.glibc
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.AudioUnit
              pkgs.darwin.apple_sdk.frameworks.CoreAudio
              pkgs.darwin.apple_sdk.frameworks.Security
            ];

            installPhase = ''
              mkdir -p $out/bin
              cp $src $out/bin/danmu
              chmod +x $out/bin/danmu
            '';

            # Fix dynamic linking on NixOS
            postFixup = pkgs.lib.optionalString pkgs.stdenv.isLinux ''
              patchelf --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" $out/bin/danmu
            '';

            meta = with pkgs.lib; {
              description = "Bilibili live room danmaku WebSocket client with TTS";
              homepage = "https://github.com/jiahaoxiang2000/blivedm_rs";
              license = licenses.mit;
              maintainers = [ ];
              platforms = platforms.unix;
            };
          };


        # Apps for easy running
        apps = {
          default = flake-utils.lib.mkApp {
            drv = self.packages.${system}.default;
            name = "danmu";
          };

          danmu = flake-utils.lib.mkApp {
            drv = self.packages.${system}.default;
            name = "danmu";
          };
        };
      });
}