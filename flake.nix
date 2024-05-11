{
  inputs.stddev.url = "github:PoolloverNathan/stddev";
  outputs = {
    self,
    stddev,
  }:
    stddev rec {
      name = "figura-backend";
      deps = pkgs: [pkgs.rustc pkgs.cargo pkgs.luajit pkgs.openssl.dev pkgs.pkg-config];
      extensions = exts: [exts.rust-lang.rust-analyzer];
      packages = system: pkgs: {
        default = let
          mf = (pkgs.lib.importTOML ./Cargo.toml).package;
        in
          pkgs.rustPlatform.buildRustPackage rec {
            pname = mf.name;
            version = mf.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
            nativeBuildInputs = deps pkgs;
            PKG_CONFIG_PATH = ["${pkgs.openssl.dev}/lib/pkgconfig/" "${pkgs.luajit}/lib/pkgconfig/"];
          };
      };
    };
}
