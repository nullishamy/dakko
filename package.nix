{
  lib,
  fetchYarnDeps,
  gtk3,
  glib-networking,
  mkYarnPackage,
  pkg-config,
  rustPlatform,
  webkitgtk,
  cairo,
  gdk-pixbuf,
  glib,
  dbus,
  openssl_3,
  librsvg,
  cargo-tauri,
  makeWrapper,
}:

let
  pname = "dakko";
  version = "0.0.1";

  frontend-build = mkYarnPackage {
    inherit version;
    pname = "dakko-ui";

    src = ./.;

    offlineCache = fetchYarnDeps {
      yarnLock = ./yarn.lock;
      sha256 = "sha256-iyIylYMqnsYTpW+D+fMs0AwrLZqcVeGN7SspyAFKyjE=";
    };

    packageJSON = ./package.json;

    buildPhase = ''
      export HOME=$(mktemp -d)
      yarn --offline build

      mkdir -p $out
      cp -r deps/dakko/build $out
    '';

    distPhase = "true";
    dontInstall = true;
  };
in

rustPlatform.buildRustPackage {
  inherit version pname;

  src = ./src-tauri;

  cargoLock = {
    lockFile = ./src-tauri/Cargo.lock;
  };

  postPatch = ''
    substituteInPlace tauri.conf.json --replace-fail '"distDir": "../build"' '"distDir": "${frontend-build}/build"'
    substituteInPlace tauri.conf.json --replace-fail '"beforeBuildCommand": "yarn build"' '"beforeBuildCommand": ""'
  '';

  buildType = "debug";

  buildPhase = ''
    runHook preBuild
    cargo tauri build
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin
    makeWrapper target/release/dakko $out/bin/dakko \
     --set WEBKIT_DISABLE_COMPOSITING_MODE 1 \
     --prefix GIO_MODULE_DIR : ${glib-networking}/lib/gio/modules/

    runHook postInstall
  '';

  buildInputs = [
    webkitgtk
    gtk3
    cairo
    gdk-pixbuf
    glib
    dbus
    openssl_3
    librsvg
    makeWrapper
    glib-networking
  ];

  nativeBuildInputs = [
    pkg-config
    cargo-tauri
  ];

  doCheck = false;

  meta = with lib; {
    description = "A [more] native[ly integrated] Fediverse client";
    homepage = "https://github.com/nullishamy/dakko";
    license = licenses.osl3;
    mainProgram = "dakko";
    maintainers = with maintainers; [ nullishamy ];
  };
}
