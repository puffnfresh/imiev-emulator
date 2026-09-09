{ lib, rustPlatform, wasm-bindgen-cli, lld }:

rustPlatform.buildRustPackage {
  pname = "imiev-wasm";
  version = "0.1.0";

  src = lib.cleanSourceWith {
    src = ../.;
    filter = path: type:
      let base = baseNameOf path; in
      base != "result"
      && base != "target"
      && base != "flake.nix"
      && base != "flake.lock"
      && base != "pkg";
  };

  cargoLock.lockFile = ../Cargo.lock;

  cargoBuildFlags = [ "-p" "imiev-wasm" "--target" "wasm32-unknown-unknown" ];
  nativeBuildInputs = [ wasm-bindgen-cli lld ];

  doCheck = false;
  auditable = false;

  installPhase = ''
    runHook preInstall

    mkdir -p "$out"
    wasm-bindgen \
      target/wasm32-unknown-unknown/release/imiev_wasm.wasm \
      --out-dir "$out" --target web

    runHook postInstall
  '';
}
