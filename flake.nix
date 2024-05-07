{
  description = "A Rust web server including a NixOS module";

  # Nixpkgs / NixOS version to use.
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/7918531";
    import-cargo.url = github:edolstra/import-cargo;
    extensions.url = "github:nix-community/nix-vscode-extensions";
  };

  outputs = {
    self,
    nixpkgs,
    import-cargo,
    extensions,
  }: let
    # System types to support.
    supportedSystems = ["x86_64-linux"];

    # Helper function to generate an attrset '{ x86_64-linux = f "x86_64-linux"; ... }'.
    forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
  in
    (builtins.foldl' (a: b: nixpkgs.lib.recursiveUpdate a b) {}) (map (system: let
        # Nixpkgs instantiated for supported system types.
        # pkgs = import ((import nixpkgs { inherit system; }).applyPatches {
        #   name = "nixpkgs-patched-vscode-serve-web";
        #   src = nixpkgs;
        #   patches = [./nixos-nixpkgs-vscode-serve-web.patch];
        #   patchFlags = ["--binary"];
        # }) {
        #   inherit system;
        #   overlays = [self.overlay];
        # };
        pkgs = import nixpkgs {
          inherit system;
          overlays = [self.overlay];
          config = {
            allowUnfree = true;
          };
        };

        # to work with older version of flakes
        lastModifiedDate = self.lastModifiedDate or self.lastModified or "19700101";

        # Generate a user-friendly version number.
        version = "${builtins.substring 0 8 lastModifiedDate}-${self.shortRev or "dirty"}";
      in {
        # A Nixpkgs overlay.
        overlay = final: prev: {
          figura-backend = with final;
            final.callPackage ({
              inShell ? false,
              ide ? false,
              web ? false,
            }:
              stdenv.mkDerivation rec {
                name = "figura-backend-${version}";

                # In 'nix develop', we don't need a copy of the source tree
                # in the Nix store.
                src =
                  if inShell
                  then null
                  else ./.;

                buildInputs =
                  [
                    rustc
                    cargo
                    gcc
                    pkg-config
                    luajit
                  ]
                  ++ (
                    if inShell
                    then
                      [
                        # In 'nix develop', provide some developer tools.
                        rustfmt
                        clippy
                        nix
                        nixd
                        bashInteractive
                      ]
                      ++ (
                        if ide
                        then [
                          nixd
                          (pkgs.vscode-with-extensions.override {
                            vscode =
                              if web
                              then pkgs.vscode
                              else pkgs.vscodium;
                            vscodeExtensions = with extensions.extensions.${system}.open-vsx-release; [
                              rust-lang.rust-analyzer
                              # jnoortheen.nix-ide
                              # bbenoist.nix
                            ];
                          })
                        ]
                        else []
                      )
                    else [
                      (import-cargo.builders.importCargo {
                        lockFile = ./Cargo.lock;
                        inherit pkgs;
                      })
                      .cargoHome
                    ]
                  );

                target = "--release";

                buildPhase = "cargo build ${target} --frozen --offline";

                doCheck = true;

                checkPhase = "cargo test ${target} --frozen --offline";

                installPhase = ''
                  mkdir -p $out
                  cargo install --frozen --offline --path . --root $out
                  rm $out/.crates.toml
                '';

                shellHook =
                  if ide
                  then ''
                    export shellHook=
                    NIX_LD_LIBRARY_PATH="${lib.makeLibraryPath [pkgs.libgnurl pkgs.curl]}";
                    NIX_LD="$(cat ${pkgs.stdenv.cc + /nix-support/dynamic-linker})";
                    ${
                      if web
                      then "code serve-web --verbose --host 0.0.0.0 --port 2352 --without-connection-token"
                      else "codium . --verbose -w"
                    }
                  ''
                  else "";
              }) {};
        };

        # Provide some binary packages for selected system types.
        packages.${system} = {
          inherit (pkgs) figura-backend;
          default = pkgs.figura-backend;
        };
        apps.${system}.default = {
          type = "app";
          program = "${pkgs.figura-backend}/bin/figura-backend";
        };

        # Provide a 'nix develop' environment for interactive hacking.
        devShells.${system} = {
          default = self.packages.${system}.figura-backend.override {
            inShell = true;
            ide = false;
          };
          ide = self.packages.${system}.figura-backend.override {
            inShell = true;
            ide = true;
          };
          web = self.packages.${system}.figura-backend.override {
            inShell = true;
            ide = true;
            web = true;
          };
        };

        formatter.${system} = pkgs.alejandra;

        # A NixOS module.
        nixosModules.figura-backend = {pkgs, ...}: {
          nixpkgs.overlays = [self.overlay];

          systemd.services.figura-backend = {
            wantedBy = ["multi-user.target"];
            serviceConfig.ExecStart = "${pkgs.figura-backend}/bin/rust-web-server";
          };
        };

        checks.${system} = {
          compiles = pkgs.figura-backend;
          check =
            pkgs.runCommand "check" {
              buildInputs = pkgs.figura-backend.buildInputs ++ [pkgs.rustfmt];
            } ''
              set -ex
              ln -s ${./src} src
              ln -s ${./Cargo.toml} Cargo.toml
              ln -s ${./Cargo.lock} Cargo.lock
              ls
              # nix --extra-experimental-features nix-command fmt -- --check
              cargo fmt --check
              cargo t --frozen --offline --no-fail-fast -j1000
              touch $out
            '';
        };
      })
      supportedSystems);
}
