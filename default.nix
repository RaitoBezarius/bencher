{ sources ? import ./npins, pkgs ? import sources.nixpkgs {}, lib ? pkgs.lib }:
rec {
  api = pkgs.rustPlatform.buildRustPackage {
    name = "bencher-api";

    src = ./.;

    nativeBuildInputs = [
      pkgs.pkg-config
    ];

    buildInputs = [
      pkgs.fontconfig
      pkgs.sqlite
    ];

    buildAndTestSubdir = "services/api";

    cargoDeps = pkgs.rustPlatform.importCargoLock {
      lockFile = ./Cargo.lock;
    };
  };

  console-rust = pkgs.rustPlatform.buildRustPackage {
    name = "bencher-valid";
    src = ./.;

    nativeBuildInputs = with pkgs; [
      wasm-pack
      wasm-bindgen-cli_0_2_100
      binaryen
      rustc.llvmPackages.lld
      pkg-config
    ];

    buildInputs = [
      pkgs.fontconfig
      pkgs.sqlite
    ];

    cargoDeps = pkgs.rustPlatform.importCargoLock {
      lockFile = ./Cargo.lock;
    };

    buildPhase = ''
      runHook preBuild

      HOME=$(mktemp -d) wasm-pack build lib/bencher_valid --target web --no-default-features --features plus,wasm

      runHook postBuild
    '';

    installPhase = ''
      runHook preInstall

      mkdir -p $out/bencher_valid/pkg
      cp -r lib/bencher_valid/pkg/ $out/bencher_valid/pkg/

      runHook postInstall
    '';

    doCheck = false; # unnecessary here.
  };

  console = pkgs.buildNpmPackage rec {
    name = "bencher-console";
    src = ./.;

    sourceRoot = "bencher/services/console";

    npmDepsHash = "sha256-1FN3VsYR0XApD5pe4bF8NBjljy6oMkrY0O7DeuhQdIw=";

    preBuild = ''
      ls ${console-rust}
      sed -i "s|../../lib/bencher_valid|${console-rust}/bencher_valid|g" astro.config.mjs
      npm run adapter node
    '';
  };

  cli = pkgs.rustPlatform.buildRustPackage {
    name = "bencher-cli";

    src = ./.;

    nativeBuildInputs = [
    ];

    buildInputs = [
    ];

    buildAndTestSubdir = "services/cli";

    cargoDeps = pkgs.rustPlatform.importCargoLock {
      lockFile = ./Cargo.lock;
    };
  };
}
